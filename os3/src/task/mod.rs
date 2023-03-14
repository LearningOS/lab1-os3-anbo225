
lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
        }; MAX_APP_NUM];
           for (i, t) in tasks.iter_mut().enumerate().take(num_app) {
               t.task_cx = TaskContext::goto_restore(init_app_cx(i));
               t.task_status = TaskStatus::Ready;
           }
           TaskManager {
               num_app,
               inner: unsafe {
                   UPSafeCell::new(TaskManagerInner {
                       tasks,
                       current_task: 0,
                   })
               },
           }
       };
}


pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

