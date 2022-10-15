## ReCodex API utilities


This folder contains Deno/web JS client for the recodex API, and some utilities

### `archiver.ts`

Usage

```
./archiver.ts --token YOUR_BEARER_TOKEN --path recodex-archive
```

It will download all your submissions from all courses into the `recodex-archive` directory.

The token can be obtained by catching some of the ReCodex's requests in DevTools / Network tab.
ReCodex also supports generating some API tokens, maybe it will also work, I don't know.

You'll need to install Deno to use it.


### `api.ts`

This is a JS module with some ReCodex API functions exposed, currently only functions needed for the archiver are implemented.
Note that ReCodex is open source and the frontend is also written in JS, so they should have a more comprehensive SDK.
This one was just easier to write than finding + understanding + using the existing implementation.
