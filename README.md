# Work in Progress

This is a work in progress, goal is to have a CLI tool that automates AppStore Connect and Google Play Console without manually having to go to the website. Eventually add JSON output to be used in CI/CD pipelines.

So far I have implemented the following features:

Apple:

- [x] Login to AppStore Connect (Not available in the CLI as a command, but implemented)
- [x] Get all apps (Not available in the CLI as a command, but implemented)
- [x] Patch "what's new" text for an app pending release (Available in the CLI as a command)

GooglePlay:

- [ ] Nothing is implemented yet

## Features on the roadmap

- [ ] Implement "what's new" for Google Play Console
- [ ] Add support for yaml configuration file
- [ ] Make it available on homebrew
- [ ] Add json output for CI/CD pipelines
- [ ] Automated tests

> **Note:** This is a personal project and not affiliated with Apple or Google in any way. Use at your own risk. ALSO I work on it when I have time, so it might take a while to get to the roadmap features.
