from random import randint
from locust import HttpLocust, TaskSet, task

class WebsiteTasks(TaskSet):
    @task(2)
    def index(self):
        self.client.get("/")

    @task(2)
    def number(self):
        self.client.get("/number/" + str(randint(0,9)))

    @task(1)
    def rows(self):
        self.client.get("/rowtest")

    @task(1)
    def textfile(self):
        self.client.get("/public/test.txt")

class WebsiteUser(HttpLocust):
    task_set = WebsiteTasks
    min_wait = 5000
    max_wait = 15000
