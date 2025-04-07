use horbo::send::send_metrics;
use horbo::horbo::{Metric, Metrics, ProtocolDefinition};
use sysinfo::{RefreshKind, System};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

mod horbo;
fn main() {
    let mut system = System::new_with_specifics(RefreshKind::everything().without_processes());
    let cpu_count: usize = system.cpus().len();
    let definition : ProtocolDefinition = match horbo::horbo::init(){
        Ok(def) => def,
        Err(e) => {
            panic!("{}", e)
        }
    };

    loop {
        system.refresh_all(); // reset system information

        let mut total:f32 = 0.0;
        for cpu in system.cpus() {
            total += cpu.cpu_usage();
        }

        let cpu_metric = Metric{
            usage: total/cpu_count as f32,
        };
        let memory_metric = Metric{
            usage: system.used_memory() as f32/system.total_memory() as f32*100 as f32,
        };

        let mut metric_items: HashMap<String, Metric> = HashMap::new();
        metric_items.insert("cpu".to_string(), cpu_metric);
        metric_items.insert("memory".to_string(), memory_metric);

        let metrics = Metrics{
            items: metric_items
        };

        let _ = send_metrics("127.0.0.1", &definition, metrics);
        thread::sleep(Duration::from_millis(50));
    }
}
