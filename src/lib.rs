use boinc_rpc::models::ProjectInfo;
use boinc_rpc::models::TaskResult;
use chrono::Duration;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Host {
    pub addr: &'static str,
    pub password: Option<&'static str>,
    projects: HashMap<Option<String>, ProjectInfo>,
    pub results: Option<Vec<TaskResult>>,
}

impl Host {
    pub fn new(addr : &'static str, password : Option<&'static str>) -> Self {
        Host {
            addr : addr,
            password : password,
            projects : HashMap::new(),
            results : None,
        }
    }
}

pub trait ModifiedResult {
    fn progress(&self) -> f64;
    fn state(&self) -> Option<String>;
    fn remaining(&self) -> f64;
    fn elapsed(&self) -> f64;
    fn remaining_as_string(&self) -> Option<String>;
    fn elapsed_as_string(&self) -> Option<String>;
}

impl ModifiedResult for TaskResult {
    fn progress(&self) -> f64 {
        let current_cpu_time = self.final_cpu_time.unwrap();
        let remaining_cpu_time = self.remaining();
        let expected_total_runtime = current_cpu_time + remaining_cpu_time;

        let progress = (current_cpu_time / expected_total_runtime) * 100.00;
        return progress;
    }

    fn remaining_as_string(&self) -> Option<String> {
        let duration = Duration::seconds(self.remaining().round() as i64);

        if duration.num_seconds() == 0 {
            return Some("--".to_string());
        }

        return duration.formatted(Some("d h:m:s".to_string()));
    }

    fn remaining(&self) -> f64 {
        return self.estimated_cpu_time_remaining.unwrap();
    }

    fn elapsed_as_string(&self) -> Option<String> {
        let duration = Duration::seconds(self.elapsed().round() as i64);

        if duration.num_seconds() == 0 {
            return Some("--".to_string());
        }

        return duration.formatted(Some("d h:m:s".to_string()));
    }

    fn elapsed(&self) -> f64 {
        return self.final_elapsed_time.unwrap();
    }

    // This returns an incorrect state - all values are currently Some(2)
    fn state(&self) -> Option<String> {
        match self.active_task {
            None => return Some("Unknown state".to_string()),
            _ => return Some("Active".to_string()),
        }
    }
}

pub trait LocalDuration {
    fn formatted(&self, format : Option<String>) -> Option<String>;
    fn calculate(&self, total : &mut i64, seconds : &mut i64, format : &String, contains : char, appender : Option<String>) -> String;
}

impl LocalDuration for Duration {
    fn formatted(&self, format : Option<String>) -> Option<String> {
        let mut formatted = String::new();
        let mut full_seconds = self.num_seconds();

        if let Some(format) = format {
            formatted.push_str(&self.calculate(&mut full_seconds, &mut 86400, &format, 'd', Some("d ".to_string())));
            formatted.push_str(&self.calculate(&mut full_seconds, &mut 3600, &format, 'h', Some(":".to_string())));
            formatted.push_str(&self.calculate(&mut full_seconds, &mut 60, &format, 'm', Some(":".to_string())));
            formatted.push_str(&self.calculate(&mut full_seconds, &mut 0, &format, 's', None));
        }

        return Some(formatted);
    }

    fn calculate(&self, total : &mut i64, seconds : &mut i64, format : &String, contains : char, appender : Option<String>) -> String {
        let mut result = String::new();

        if format.contains(contains) && total >= seconds {
            let unit;
            if seconds == &mut 0 {
                unit = *total;
            } else {
                unit = ((*total / *seconds) as f64).round() as i64;
            }
            *total -= unit * *seconds;

            result.push_str(&format!("{:02}", unit));
            if let Some(appender) = appender {
                result.push_str(&appender);
            }
        }

        return result;
    }
}
