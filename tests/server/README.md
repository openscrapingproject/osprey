# Mock server

This folder contains the files necessary to run `json-server` to create a mock Jobs API server.

Changes:
- we removed the jobs_collection field: the API executor should not do the job of reconciling/creating jobs from jobs collections. This should be done either by the client before submission or by the API server itself. For now, client will do it.