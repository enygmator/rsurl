# rsurl

This is project is aimed at creating a URL shortening service.  
`rsurl` stands for **rusty-short-url**.

> We DO NOT use any 3rd paty API and all frameworks/libraries used are open-source and can be substituted with closed source versions (since they are all `MIT Licensed` - almost?).

## Overview

> I shall use this file for temporary documentation

### The requirements

- URL shortener service
- custom rsurl
- store in DB
- statistics obtain (backend)
- display statistics (frontend)
- provide possible mobile app using MAUI

#### Hackathon Actual SRS

1. **Basics**: UI alignment, basic hygiene, usability and more
2. **Signup/Login experience**: Would you have a login/signup experience? How would one sign up? E-Mail and password? What is your favorite sign-in method?
3. **URL Management**: Can a user edit a shortened URL? What can they edit? Can it be deleted? Can users choose a custom short URL string?
4. **Statistics**: Can you present a Geographical breakup of URL access? What devices and browsers are being used to access the URL? What is the screen size? Mode info?
5. **Exceptions**: Server can be down; user can lose internet connectivity, what else?

> The above-mentioned are just examples and is not a comprehensive list.

## Architecture

### The service `"backend"`

[speed reference](https://www.techempower.com/benchmarks/#section=data-r20&hw=ph&test=fortune)

The shortening service need to be super-fast and is therefore built on a systems programming language called `Rust`; which has speeds as fast as C, but everything else is as elegant as Java or Python.

There are lots of other reasons as to why I chose RUst (package manager, highly secure, ...)

#### Link

Bitly (largest URL Shortening) shortens 600 million links per month.

Bitly links cannot be changed once created.

I will Use `HTTP 301` **moved permanently** redirect for those links that people don't intend to ever change (These redirections are meant to last forever).  
It makes more sense to have a new URL point to another location than edit a previous one.

> maybe users can set expiration time

A custom URL may be chosen. People can edit and delete redirects if they want - but for that, they'll have to choose that option beforehand, which will lead to a `HTTP 302` **moved temporarily** redirect.

Mozilla has good [documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/Redirections).

I'm assuming that multiple sURLs can point to the same URL and therefore the sURL will be randomly generated.

#### Storage

I'm using key-value store for the shortened URLs.

I'm using key-value store for user data.


We'll assume we have 1 billion links per month; we'll operate for 10 years = 120 billion links.  
Therefore, URL with length 7, will give 64^7 = ~5 Trillion URLs.

[Why MongoDB](https://www.mongodb.com/databases/key-value-database)

It is more than a simple key-value store database, but we'll use it as one.

I thought of using redis as an in-memory cache, but it seems like mongoDB is a **FREE** key-value store that has this caching feature inbuilt. So, the latency of read-miss is reduced as it's implemented internally.

I'm not sure if it's open source as the `server` is open source on GitHub, but I don't know if that's the whole of it. As far as I can see, it is open source with a custom license.

I'm using a columnar store for analytics.

#### Analytics

- country
- browser
- user-agent
- device
- screen size
- mode info
- HTTP Request type
- datetime
- language
- referrer

### The UI `"frontend"`

For this, I chose `C#` because it is elegant to write code in it. The web framework I'm using is called **ASP.NET Core 6 Blazor Server**, which is a webserver in C# that uses C# both on the client and the server and is implemented in such a way so as to offer the best live update experience possible. It is an enterprise-level open source framework from Microsoft that's meant to scale well (among other things).

C# is technically slower that rust (being a compiled language), but we can live with that as the UI need not be as fast as the service itself.

This framework is actually about 5 to 10 times faster than `javascript` and `python` based web frameworks (since C# is statically compiled). The first time load time seems to be slow, but after that, it is super-fast.


## Product design Decisions

There are a lot of reasons I architected this app the way I did:

1. Blazor server: realtime connection, transient service = network load optimization; best diff based sync - multi-device (global) sync; same codebase for cross-platform mobile apps; faster than JS/python.

2. Rust +  : secure; as fast as C; like Java/C#/python 

3. Base62 is used because: it most commonly used; higher encoding creates characters that might be interpreted as something other than string.

4. Link traffic: -



## Extensibility

### Mobile apps

Using dotNET MAUI (open-source, enterprise supported, cross-platform app framework from Microsoft), we have a version of this website for the app.

This DOES NOT mean that there will be a `webview` which will show the website, but rather, it'll be like `electron`, but much faster.

All the components of the website can be reused inside the cross-platform app, guaranteeing a single code base and nearly full re-usability.

