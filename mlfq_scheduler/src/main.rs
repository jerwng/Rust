mod mlfq;

use mlfq::MLFQ;
use mlfq::Process;


fn main() {
    println!("MLFQ Scheduler Implementation");
    
    // You can add any demonstration code here if you wish
    let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
    let process1 = Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 };
    let process2 = Process { id: 2, priority: 1, remaining_time: 10, total_executed_time: 0 };
    let process3 = Process { id: 3, priority: 5, remaining_time: 10, total_executed_time: 0 };
    let process4 = Process { id: 4, priority: 0, remaining_time: 10, total_executed_time: 0 };

    mlfq.add_process(process1);
    mlfq.add_process(process2);
    mlfq.add_process(process3);
    mlfq.add_process(process4);

    mlfq.execute_process(2);
}