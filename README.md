# Task Manager

An offline, lightweight, and minimalistic-ish planner. Just trying out the Tauri backend with React frontend and Jest testing framework to put a personal spin on the ubiquitous planning app.

## Features
* TBD

## How to Use?
* Haven't gotten to this stage yet.

## Development Resources
* I followed the [Tauri quickstart with Vite](https://tauri.app/v1/guides/getting-started/setup/vite), choosing React as my frontend framework.
* I configured the tests with Testing-Library React and Jest using this [guide](https://www.pluralsight.com/guides/how-to-test-react-components-in-typescript)
  * Should be noted that the guide was slightly outdated: [the most recent version of Jest shipped the jsdom separately](https://stackoverflow.com/questions/72013449/upgrading-jest-to-v29-error-test-environment-jest-environment-jsdom-cannot-be)
  * also the option `"@testing-library/react/cleanup-after-each",` is [seemingly not needed](https://kentcdodds.com/blog/common-mistakes-with-react-testing-library#using-cleanup)
* I configured ESLint for TypeScript and JSDoc:
  * [ESLint rules](https://eslint.org/docs/latest/rules)
  * [Typescript ESLint rules](https://typescript-eslint.io/rules)
  * [JSDoc ESLint rules](https://www.npmjs.com/package/eslint-plugin-jsdoc#user-content-eslint-plugin-jsdoc-rules)

