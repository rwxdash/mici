The initial idea for this tool came into existence from the problems we were having with our Slackbot at Thundra.

The said Slackbot is running on some EC2 machine, playing an interface for our deployments. The application is written in Java and is quite complex, as well as cluttered. So much so that we can't really operate or make any improvements on that. The most improvement we've done is to manage it setup with an AWS CDK Stack. The application contains a Slack integration, OpsGenie integration, a job processor to run multiple commands asynchoronously, logging system where it logs the whole process and uploads to S3, role management, a lambda invoker, database migration management, and many more in one application.

Apart from all these complexity, if Slackbot goes down for some reason our deployment process became stuck. The biggest problem is that it can't update itself. So, when there is new changes, we have to release it manually and restarting the process inside the machine.

Thinking this and other problems, I wanted to separate all these functionalities and develop a job runner that can also be accessible from the terminal. A CLI that generates its commands on runtime, dynamically. Having familiarity with CIs like GitHub Action, CircleCI, etc., it seemed OK to generate these commands from YAML files that are similar to workflow definitions.

This application will only run the given commands and won't worry about Slack, or any other integration for the moment. Although, any abstraction layer to this can be implemented. So, the application should be designed with this in mind.
