extern crate chrono;

pub trait ModifiedResult {
    fn progress(&self) -> String;
    fn time_left(&self) -> Option<String>;
    fn state(&self) -> Option<String>;
}

impl ModifiedResult for rpc::models::Result {
    fn progress(&self) -> String {
        let current_cpu_time = self.final_cpu_time.unwrap();
        let remaining_cpu_time = self.estimated_cpu_time_remaining.unwrap();
        let expected_total_runtime = current_cpu_time + remaining_cpu_time;

        let progress = (current_cpu_time / expected_total_runtime) * 100.00;
        return format!("{0:.2} %", progress);
    }

    /**
     * Need to fix duration format so this can be used on the pane
     */
    fn time_left(&self) -> Option<String> {
        let duration = chrono::Duration::seconds(self.estimated_cpu_time_remaining.unwrap().round() as i64);
        println!("{:?}", duration);
        return Some("".to_string());
    }

    // This returns an incorrect state - all values are currently Some(2)
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
