# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform web
```

### Docker

```bash
docker build -t todo-app .
docker run --rm --detach --network host --name toto-app todo-app
```
