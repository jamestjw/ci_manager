# ci_manager
This is a CLI tool to help manage CI between Github and Circle-CI. 

# Features
## Check status of CI workflow for a particular Github branch.
Suppose that you want to find out about the status for the `master` branch.
```
ci_manager status master
```

## Approve job in the workflow to allow CI to proceed
Suppose that you want to approve a pending job in the workflow related to the `master` branch
```
ci_manager approve master
```

# Configuration
To set things up, make a copy of `config.example` at `~/.ci_manager/config` and fill it in with the relevant information.
```
mkdir -p ~/.ci_manager
cp config.example ~/.ci_manager/config
```
