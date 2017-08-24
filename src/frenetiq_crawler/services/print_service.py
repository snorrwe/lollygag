import threading
from frenetiq_crawler.dependency_injection.inject import Inject
from frenetiq_crawler.dependency_injection.requirements import HasAttributes, HasMethods

LOGLEVELS = {
    "all": 0,
    "debug": 1,
    "info": 2,
    "warn": 3,
    "error": 4,
    "none": 5
}

class PrintService(object):
    """
    A logging service that logs to the standard output via print
    """
    config = Inject("config_service", HasAttributes("loglevel"))
    output = Inject("logging_output", HasMethods("write"))

    def __init__(self):
        self.loglevel = LOGLEVELS[self.config.loglevel]

    def debug(self, *messages):
        self.log(LOGLEVELS["debug"], "\n[Debug]", *messages)

    def info(self, *messages):
        self.log(LOGLEVELS["info"], "\n[Info]", *messages)

    def error(self, *messages):
        self.log(LOGLEVELS["error"], "\n[Error]", *messages)

    def warn(self, *messages):
        self.log(LOGLEVELS["warn"], "\n[Warn]", *messages)

    def log(self, level, prefix, *messages):
        """
        Checks if the requested loglevel is below the available loglevel
        If so, extend the message with the name of the current thread and call write
        """
        if self.loglevel > level:
            return
        msg = "%sThread=[%s]\t" % (prefix, threading.currentThread().name)
        for message in messages:
            msg += " %s" % message
        self.write(msg)

    def write(self, msg):
        self.output.write(msg)
