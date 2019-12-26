extern crate chrono;

pub trait ModifiedResult {
    fn progress(&self) -> f64;
    fn time_left(&self) -> Option<String>;
    fn state(&self) -> Option<String>;
}

impl ModifiedResult for rpc::models::Result {
    fn progress(&self) -> f64 {
        let current_cpu_time = self.final_cpu_time.unwrap();
        let remaining_cpu_time = self.estimated_cpu_time_remaining.unwrap();
        let expected_total_runtime = current_cpu_time + remaining_cpu_time;

        return (current_cpu_time / expected_total_runtime) * 100.00;
    }

    /**
     * Need to fix duration format so this can be used on the pane
     */
    fn time_left(&self) -> Option<String> {
        let duration = chrono::Duration::seconds(self.estimated_cpu_time_remaining.unwrap().round() as i64);
        return Some("".to_string());
    }

    fn state(&self) -> Option<String> {
        match self.state {
            Some(0) => return Some("Uninitialized".to_string()),
            Some(1) => return Some("Executing".to_string()),
            Some(2) => return Some("Suspended".to_string()),
            Some(3) => return Some("Abort pending".to_string()),
            Some(4) => return Some("Quit pending".to_string()),
            Some(5) => return Some("Copy pending".to_string()),
            _ => return Some("Unknown state".to_string())
        }
    }
}
