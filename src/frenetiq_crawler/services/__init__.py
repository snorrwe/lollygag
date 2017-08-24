import sys
import argparse
import threading
import requests
from frenetiq_crawler.crawler.link_crawler import LinkCrawler
from frenetiq_crawler.services.config_service import ConfigService
from frenetiq_crawler.services.print_service import PrintService
from frenetiq_crawler.services.work.work_service import WorkService
from frenetiq_crawler.dependency_injection.inject import Inject
try:
    import Queue
except ImportError:
    import queue as Queue

SERVICES = {
    'requests': requests,
    'crawler_factory': LinkCrawler,
    'argparse': argparse.ArgumentParser,
    'config_service': ConfigService,
    'log_service': PrintService,
    'work_service':  WorkService,
    'threading':  threading,
    'queue': Queue.Queue,
    'logging_output': sys.stdout
}

def get_required_service_keys():
    return SERVICES.keys()

def register_services(services=None):
    if not services:
        services = SERVICES
    Inject.register_features(**services)
