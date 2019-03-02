devproxy [WIP]
==============

Devproxy is a transparent HTTP proxy to reroute HTTP requests, e.g. when developing in micro-service setups.

Devproxy is WIP.

Example
-------

You run service "*my-awesome-frontend*" locally. *my-awesome-frontend* relies on several backends, e.g. "*my-inventory*" 
and *my-authorization-service*, and the local setup is configured to talk tho *my-authorization-service.testing.example.com* 
and *my-inventory.testing.example.com*.

You're fine with this setup, as you're just tinkering around with the layout. Now you want to test some edge-cases, i.e. 
very large inventory-lists (as returned by *my-inventory*). That's where **devproxy** comes into play. You configure a 
mapping for *my-inventory* -> *localhost:8180*, *devproxy* sends any request for *my-inventory.testing.example.com* to
*localhost:8180* where you can run a fake-server that returns the data you want to test. 



