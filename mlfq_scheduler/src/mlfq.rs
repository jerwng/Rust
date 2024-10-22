// src/mlfq.rs

#[derive(Clone)]
pub struct Process {
    pub id: u32,
    pub priority: usize,  // Represents the current queue index
    pub remaining_time: u32,
    pub total_executed_time: u32,
}

pub struct MLFQ {
    queues: Vec<Vec<Process>>,
    num_levels: usize,
    time_quanta: Vec<u32>,
    current_time: u32,
}

impl MLFQ {
    pub fn new(num_levels: usize, time_quanta: Vec<u32>) -> Self {
        MLFQ {
            queues: vec![Vec::new(); num_levels],
            num_levels,
            time_quanta,
            current_time: 0,
        }
    }

    // Exercise 1: Queue Management
    pub fn add_process(&mut self, process: Process) {
        // Add the process to the appropriate queue based on its priority
        // Ensure the priority is within the valid range (0 to num_levels - 1)

        let mut priority: usize = process.priority;

        // Out of range priorities gets placed in lowest priority queue
        if priority > self.num_levels - 1 {
            priority = self.num_levels - 1;
        }

        self.queues[priority].push(process);
    }

    // Exercise 2: Process Execution
    pub fn execute_process(&mut self, queue_index: usize) {
        // Execute the process for its time quantum or until completion
        // Update remaining_time, total_executed_time, and current_time
        // Move the process to a lower priority queue if it doesn't complete

        let queue = &mut self.queues[queue_index];
        let queue_time_quanta = self.time_quanta[queue_index];

        if queue.is_empty() {
            return;
        }

        // Safe to .unwrap() here since queue is verified to have a Process
        let mut process = queue.remove(0);

        // Process is completed during this run
        if process.remaining_time <= queue_time_quanta {
            process.total_executed_time += process.remaining_time;
            process.remaining_time = 0;
            return;
        }
        
        // Process is not completed at the end of time quanta, move to lower priority 
        // if it is not already in the lowest priority queue
        let process_next_priority = if queue_index + 1 < self.num_levels {
            queue_index + 1
        } else {
            queue_index
        };

        process.total_executed_time += queue_time_quanta;
        process.remaining_time -= queue_time_quanta;

        process.priority = process_next_priority;
        self.queues[process_next_priority].push(process);
    }

    // Exercise 3: Priority Boost
    pub fn priority_boost(&mut self) {
        // Move all processes to the highest priority queue
        // Reset the priority of all processes to 0
        for i in 1..self.num_levels {
            while !self.queues[i].is_empty() {
                let process = self.queues[i].remove(0);
                self.queues[0].push(process);
            }
        }
    }

    // Simulate time passing and trigger a boost if needed
    pub fn update_time(&mut self, elapsed_time: u32) {
        self.current_time += elapsed_time;
        let boost_interval = 100;
        if self.current_time % boost_interval == 0 {
            self.priority_boost();
        }
    }
}

// Automated Test Cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        
        let process1 = Process { id: 1, priority: 0, remaining_time: 10, total_executed_time: 0 };
        let process2 = Process { id: 2, priority: 1, remaining_time: 5, total_executed_time: 0 };
        let process3 = Process { id: 3, priority: 5, remaining_time: 8, total_executed_time: 0 };

        mlfq.add_process(process1);
        mlfq.add_process(process2);
        mlfq.add_process(process3);

        assert_eq!(mlfq.queues[0].len(), 1);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[2].len(), 1);
    }

    #[test]
    fn test_execute_process() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[0].push(Process { id: 1, priority: 0, remaining_time: 5, total_executed_time: 0 });

        mlfq.execute_process(0);

        assert_eq!(mlfq.queues[0].len(), 0);
        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[1][0].remaining_time, 3);
        assert_eq!(mlfq.queues[1][0].total_executed_time, 2);
    }

    #[test]
    fn test_priority_boost() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        mlfq.queues[2].push(Process { id: 2, priority: 2, remaining_time: 3, total_executed_time: 7 });

        mlfq.update_time(100); // Should trigger priority boost

        assert_eq!(mlfq.queues[0].len(), 2);
        assert_eq!(mlfq.queues[1].len(), 0);
        assert_eq!(mlfq.queues[2].len(), 0);
    }

    #[test]
    fn test_boost_does_not_occur_prematurely() {
        let mut mlfq = MLFQ::new(3, vec![2, 4, 8]);
        mlfq.queues[1].push(Process { id: 1, priority: 1, remaining_time: 5, total_executed_time: 3 });
        
        mlfq.update_time(50); // No boost should happen

        assert_eq!(mlfq.queues[1].len(), 1);
        assert_eq!(mlfq.queues[0].len(), 0);
    }
}