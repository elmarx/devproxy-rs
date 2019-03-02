"use strict";

const express = require("express");

const app = express();

app.get("/redirect", (req, res) => res.redirect(302, "/"));
app.get("/slow", (req, res) => {
    for(let i = 1; i < 6; i++) {
        setTimeout(() => {
            res[i == 5 ? "end" : "write"](`${i}\n`);
        }, i * 500);
    }
});
app.get("/header", (req, res) => {
    res.set("upstream", "42");
    res.send("Sent response header 'upstream: 42'");
});

app.use((req, res) => {
    res.status(200).write(`${req.method} http://${req.headers.host}${req.url}\n`);
    res.end(JSON.stringify(req.headers, null, 4));
});

app.listen(8180);
app.listen(8280);

