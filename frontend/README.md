# PizzaDAO App Frontend

This folder contains everything you need to run your app locally, except some requisites that must be installed in your sistem.

## Run Locally

**Prerequisites:**  Node.js v22.18.0 and NPM v10.9.3.

### Installing node

```bash
# Download and install nvm:
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash

# in lieu of restarting the shell
\. "$HOME/.nvm/nvm.sh"

# Download and install Node.js:
nvm install 22

# Verify the Node.js version:
node -v # Should print "v22.18.0".
nvm current # Should print "v22.18.0".

# Verify npm version:
npm -v # Should print "10.9.3".
```

### Setting up dependencies

1. Install dependencies:

```npm install```

3. Set the `GEMINI_API_KEY` in [.env.local](.env.local) to your Gemini API key as follows:

```GEMINI_API_KEY=PLACEHOLDER_API_KEY```

4. Run the app:

```npm run dev```

That’s it! The app should be running correctly in port 5173 (VITE reserved port) `http://localhost:5173`.


## Disponibilizing the development app for everyone with testing purposes

> ⚠️ Caution: This procedure is not secure as it exposes an application in development mode through the web from your computer. Use it at your own risk.

### 1. Run your application at port 8080

Execute:
```bash
npm run dev -- --port 8080
```

### 2. Install ngrok

Install ngrok with `npm`.
```bash
npm install -g ngrok
```

### 3. Expose the port with ngrok

Run:

```bash
ngrok http 8080
```
Ngrok is goign to give you an URL like the following:

```
https://abc123.ngrok.io
```
