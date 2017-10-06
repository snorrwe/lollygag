from threading import Lock
from lollygag.dependency_injection.inject import Inject
from lollygag.dependency_injection.requirements import HasMethods, HasAttributes


def get_labels(count, collection):
    current = 0
    for _ in range(count):
        while current in collection:
            current += 1
        yield current


class WorkService(object):
    __worker_labels = set()
    __instances = 0
    __init_lock = Lock()

    threading = Inject("threading", HasMethods("Thread"))
    config_service = Inject("config_service", HasAttributes("threads"))
    log_service = Inject("log_service", HasMethods("debug", "info"))
    queue_factory = Inject("queue", return_factory=True)

    def __init__(self):
        worker_count = int(self.config_service.threads)
        assert worker_count > 0, "Thread count cannot be less than 1!"
        WorkService.__init_lock.acquire(True)
        try:
            self.__active_count = 0
            self.__worker_labels = set()
            self.queue = self.queue_factory()
            self.__init_workers(worker_count, WorkService.__instances)
            WorkService.__instances += 1
        finally:
            WorkService.__init_lock.release()

    def __init_workers(self, number, instance_number):
        label = max(WorkService.__worker_labels) if WorkService.__worker_labels else 0
        for _ in range(number):
            label += 1
            self.__worker_labels.add(label)
            WorkService.__worker_labels.add(label)
            worker = self.threading.Thread(target=self.__worker,
                                           name="WSc[%s]--%s" % (instance_number, label))
            worker.daemon = True
            worker.start()

    def __del__(self):
        for label in self.__worker_labels:
            WorkService.__worker_labels.remove(label)

    def request_work(self, target, blocking=True):
        assert target is not None
        assert callable(target)
        return self.queue.put(target, blocking)

    def terminate_all(self, graceful=False):
        self.log_service.debug("-----------------Terminate request received-----------------")
        if graceful:
            self.queue.join()

    def active_count(self):
        return self.__active_count

    def __worker(self):
        while 1:
            task = self.queue.get()
            self.__active_count += 1
            task()
            self.__active_count -= 1
            self.queue.task_done()
