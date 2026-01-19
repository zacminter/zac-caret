use std::process::{Command, Child, Stdio};
use std::path::PathBuf;
use std::io::Write;
use uuid::Uuid;

pub struct ClaudeCliManager {
    pub working_dir: PathBuf,
    pub active_processes: Vec<ClaudeProcess>,
}

pub struct ClaudeProcess {
    pub id: String,
    pub worker_id: String,
    pub mission_id: String,
    pub child: Child,
    pub started_at: std::time::Instant,
}

impl ClaudeCliManager {
    pub fn new(working_dir: PathBuf) -> Self {
        Self {
            working_dir,
            active_processes: Vec::new(),
        }
    }

    /// Spawn a Claude Code CLI instance for a mission
    pub fn spawn_for_mission(
        &mut self,
        worker_id: String,
        mission_id: String,
        project_path: &str,
        mission_file: &str,
    ) -> Result<String, String> {
        let process_id = Uuid::new_v4().to_string();

        println!("🚀 Spawning Claude CLI for mission: {}", mission_file);
        println!("   Working dir: {}", project_path);

        // Build command
        let child = Command::new("claude-code")
            .arg("--dangerously-skip-permissions")
            .arg(mission_file)
            .current_dir(project_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn claude-code: {}", e))?;

        let process = ClaudeProcess {
            id: process_id.clone(),
            worker_id,
            mission_id,
            child,
            started_at: std::time::Instant::now(),
        };

        self.active_processes.push(process);

        Ok(process_id)
    }

    /// Check if any processes have completed
    pub fn check_completions(&mut self) -> Vec<CompletionResult> {
        let mut completed = Vec::new();
        let mut still_running = Vec::new();

        for mut process in self.active_processes.drain(..) {
            match process.child.try_wait() {
                Ok(Some(status)) => {
                    // Process completed
                    let duration = process.started_at.elapsed();

                    // Try to read stdout
                    let output = process.child.wait_with_output().ok();

                    let result = CompletionResult {
                        worker_id: process.worker_id,
                        mission_id: process.mission_id,
                        success: status.success(),
                        duration_secs: duration.as_secs(),
                        output: output.map(|o| String::from_utf8_lossy(&o.stdout).to_string()),
                    };

                    completed.push(result);
                }
                Ok(None) => {
                    // Still running
                    still_running.push(process);
                }
                Err(e) => {
                    eprintln!("Error checking process: {}", e);
                    still_running.push(process);
                }
            }
        }

        self.active_processes = still_running;
        completed
    }

    /// Send additional instructions to a running worker
    pub fn send_message(&mut self, process_id: &str, message: &str) -> Result<(), String> {
        let process = self.active_processes.iter_mut()
            .find(|p| p.id == process_id)
            .ok_or("Process not found")?;

        if let Some(stdin) = process.child.stdin.as_mut() {
            stdin.write_all(message.as_bytes())
                .map_err(|e| format!("Failed to write to stdin: {}", e))?;
            stdin.write_all(b"\n")
                .map_err(|e| format!("Failed to write newline: {}", e))?;
            stdin.flush()
                .map_err(|e| format!("Failed to flush: {}", e))?;

            println!("📨 Sent message to worker: {}", message);
            Ok(())
        } else {
            Err("Process has no stdin".to_string())
        }
    }
}

pub struct CompletionResult {
    pub worker_id: String,
    pub mission_id: String,
    pub success: bool,
    pub duration_secs: u64,
    pub output: Option<String>,
}

impl CompletionResult {
    /// Parse tokens used from output
    pub fn extract_tokens(&self) -> u32 {
        if let Some(output) = &self.output {
            // Look for pattern like "Tokens used: 4521" or similar
            // This is a simplified parser - adjust based on actual Claude CLI output
            if let Some(start) = output.find("tokens") {
                let snippet = &output[start..];
                if let Some(num_start) = snippet.find(char::is_numeric) {
                    let num_str: String = snippet[num_start..]
                        .chars()
                        .take_while(|c| c.is_numeric())
                        .collect();
                    return num_str.parse().unwrap_or(0);
                }
            }
        }

        // Default estimate if not found
        1000
    }

    /// Extract completion summary from output
    pub fn extract_summary(&self) -> String {
        if let Some(output) = &self.output {
            // Look for [DONE] lines
            let done_lines: Vec<&str> = output.lines()
                .filter(|line| line.contains("[DONE]"))
                .collect();

            if !done_lines.is_empty() {
                return done_lines.join("\n");
            }

            // Fallback: last 5 lines
            let lines: Vec<&str> = output.lines().collect();
            let start = lines.len().saturating_sub(5);
            return lines[start..].join("\n");
        }

        "Task completed".to_string()
    }
}
