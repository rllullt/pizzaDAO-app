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

2. Set the `GEMINI_API_KEY` in [.env.local](.env.local) to your Gemini API key

3. Run the app:
```npm run dev```

That’s it! The app should be running correctly in port 5173 (VITE reserved port) `http://localhost:5173`.


## **🌐 Ahora puedes acceder a tu aplicación:**

### **Opción 1: Navegador Web**
Abre tu navegador y ve a:
```
http://localhost:8080/
```

### **Opción 2: Desde Cursor/VS Code**
1. **Presiona `Ctrl + Click`** en la URL `http://localhost:8080/` que aparece en la terminal
2. O **copia y pega** la URL en tu navegador

### **Opción 3: Usar la configuración de Debug**
1. Ve a la pestaña **"Run and Debug"** (Ctrl+Shift+D)
2. Selecciona **"Launch Chrome against localhost"**
3. Presiona **F5**

## **🎯 Lo que deberías ver:**

Una vez que abras la URL, verás la aplicación **Global Pizza Party 2026** con:

- 🍕 Pantalla de login con el logo de pizza
- 📧 Campo para ingresar email
- 🎨 Diseño oscuro con acentos rojos y amarillos

## **🔄 Hot Reload Activo:**

Ahora cualquier cambio que hagas en el código se reflejará automáticamente en el navegador. ¡No necesitas refrescar la página!

**¿Ya puedes ver la aplicación en tu navegador?** Si tienes algún problema para acceder, dime qué error ves.

## **📱 Para obtener la URL de tu app:**

### **Paso 1: Ejecuta tu aplicación**
En tu terminal, ejecuta:
```bash
npm run dev -- --port 8080
```

### **Paso 2: Instalar ngrok**
En otra terminal nueva, ejecuta:
```bash
npm install -g ngrok
```

### **Paso 3: Exponer con ngrok**
En la misma terminal donde instalaste ngrok, ejecuta:
```bash
ngrok http 8080
```

## ** Lo que necesito de ti:**

**Ejecuta estos comandos y dime qué URL te aparece.**

Ngrok te dará una URL como:
```
https://abc123.ngrok.io
```

**Esa será la URL que podrás usar en tu celular.**

## ** Si no quieres ejecutar comandos:**

Puedo ayudarte a subir la app a un servicio gratuito como:
- **Netlify** (más fácil)
- **Vercel** 
- **GitHub Pages**

**¿Prefieres que te ayude a subirla a un servicio gratuito o quieres ejecutar los comandos para obtener la URL local?**

Dime qué opción prefieres y te guío paso a paso.
