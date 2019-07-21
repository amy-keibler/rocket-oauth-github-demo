# Rocket-OAuth-GitHub-Demo

[Deployed on Heroku here](https://rocket-oauth-github-demo.herokuapp.com/)

This project followed the [GitHub Rails OAuth Guide](https://developer.github.com/v3/guides/basics-of-authentication/), but in Rust using the Rocket framework. The end result is a simple web application that requests access to a GitHub user's information and displays it.

My goal was to become more familiar with how to use OAuth to create applications that are secure and that don't require direct password management/storage. The application does not concern itself with consuming the actual API beyond getting the user's information, but it would not be too difficult to extend the example to use the token on the authenticated user.

Ideally, Rocket will eventually have something similar to [Überauth](https://github.com/ueberauth/ueberauth) that handles authentication automatically via a variety of authentication providers.

## OC, Do Not Steal

This is just a demo application that I wrote for learning purposes. Please do not copy-paste this code into a production application without auditing it yourself. The standard legal disclaimer of no-implied-warranty applies.

All that aside, hopefully you find this interesting and/or useful.

## HTTPS for `localhost`

I followed the [Lets Encrypt localhost Guide](https://letsencrypt.org/docs/certificates-for-localhost/) and put the certs in a `certs/` folder in the root of this project.

Currently, Rocket does not appear to be able to configure TLS for just dev and to let a different service provide SSL termination in production. The repository is in the production state so that it can be automatically deployed to Heroku. For local dev, uncomment the line that adds the `tls` feature to Rocket.

## SameSite Lax v. Strict

Firefox was not sending the cookies for the initial redirect, which caused the page to show the login section. Since a refresh worked, the auth flow had succeeded. I could not find any documentation as to why the cookies were not forwarded, but I suspected that it was due to a security policy. After a good amount of searching and testing, I figured out that the issue was related to [this bug](https://bugzilla.mozilla.org/show_bug.cgi?id=1465402).

From what I have gathered, Firefox considers the request to have originated as part of a chain from GitHub rather than from the rocket site, so it does not forward the cookies. I'm not _entirely_ sure what the security implications of `Strict` v. `Lax` are yet, but `Lax` allows sending cookies from requests that originate elsewhere [[relevant MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies#SameSite_cookies)]. Since the cookies are `HttpOnly` with a domain specified, they are not available to Javascript and the cookies are restricted to the domain of the application. All of the solutions I found online involved using simulated redirects rather than HTTP redirects, which seem suboptimal. If anyone has further information, I'd be interested in reading more.
