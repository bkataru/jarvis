# Deployment Guide

This guide covers deploying JARVIS to various platforms.

## Building for Production

### 1. Build the WASM Bundle

```bash
trunk build --release
```

This creates an optimized build in the `dist/` directory with:
- Minified JavaScript
- Optimized WASM binary
- Compressed assets
- Cache-friendly filenames

### 2. Verify the Build

```bash
cd dist
python3 -m http.server 8000
```

Visit `http://localhost:8000` to test the production build.

## Deployment Options

### Static Hosting (Recommended)

JARVIS is a pure static site and can be deployed to any static hosting service.

#### GitHub Pages

1. Create `.github/workflows/deploy.yml`:

```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [main]
  workflow_dispatch:

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      
      - name: Install Trunk
        run: cargo install trunk
      
      - name: Build
        run: trunk build --release --public-url /${{ github.event.repository.name }}
      
      - name: Deploy
        uses: peaceiris/actions-gh-pages@v4
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./dist
```

2. Enable GitHub Pages in repository settings (deploy from `gh-pages` branch)

#### Vercel

1. Install Vercel CLI:
```bash
npm install -g vercel
```

2. Create `vercel.json`:
```json
{
  "buildCommand": "cargo install trunk && trunk build --release",
  "outputDirectory": "dist",
  "installCommand": "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && rustup target add wasm32-unknown-unknown"
}
```

3. Deploy:
```bash
vercel --prod
```

#### Netlify

1. Create `netlify.toml`:
```toml
[build]
  command = "cargo install trunk && trunk build --release"
  publish = "dist"

[build.environment]
  RUST_VERSION = "1.70"

[[headers]]
  for = "/*"
  [headers.values]
    Cross-Origin-Embedder-Policy = "require-corp"
    Cross-Origin-Opener-Policy = "same-origin"
```

2. Deploy via Netlify CLI or drag-and-drop the `dist/` folder

#### Cloudflare Pages

1. Build locally:
```bash
trunk build --release
```

2. Deploy via Wrangler:
```bash
wrangler pages deploy dist
```

Or use the Cloudflare Pages dashboard to connect your repository.

### Custom Server

For custom server deployment (e.g., Apache, Nginx):

#### Nginx Configuration

```nginx
server {
    listen 80;
    server_name jarvis.example.com;
    root /var/www/jarvis/dist;
    index index.html;

    # Required for WebAssembly
    types {
        application/wasm wasm;
    }

    # Enable CORS if needed for MCP servers
    add_header Cross-Origin-Embedder-Policy "require-corp" always;
    add_header Cross-Origin-Opener-Policy "same-origin" always;

    # SPA routing - redirect all to index.html
    location / {
        try_files $uri $uri/ /index.html;
    }

    # Cache static assets
    location ~* \.(js|css|wasm|svg|png|jpg|jpeg|gif|ico)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # Gzip compression
    gzip on;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/wasm;
}
```

#### Apache Configuration

Create `.htaccess` in the `dist/` directory:

```apache
<IfModule mod_rewrite.c>
  RewriteEngine On
  RewriteBase /
  
  # Don't rewrite existing files
  RewriteCond %{REQUEST_FILENAME} !-f
  RewriteCond %{REQUEST_FILENAME} !-d
  
  # SPA routing
  RewriteRule ^ index.html [QSA,L]
</IfModule>

<IfModule mod_headers.c>
  # Required for WebAssembly/Workers
  Header set Cross-Origin-Embedder-Policy "require-corp"
  Header set Cross-Origin-Opener-Policy "same-origin"
</IfModule>

<IfModule mod_mime.c>
  # WASM MIME type
  AddType application/wasm .wasm
</IfModule>

<IfModule mod_expires.c>
  # Cache static assets
  ExpiresActive On
  ExpiresByType application/wasm "access plus 1 year"
  ExpiresByType application/javascript "access plus 1 year"
  ExpiresByType text/css "access plus 1 year"
</IfModule>
```

## Performance Optimization

### 1. Enable Compression

Ensure your server serves `.wasm` files with gzip or brotli compression.

### 2. CDN Integration

Use a CDN like Cloudflare or AWS CloudFront for:
- Global distribution
- Automatic compression
- DDoS protection
- Free SSL certificates

### 3. Model Caching

For production, implement proper model caching:

```rust
// Use Cache API for model storage
const MODEL_CACHE_NAME: &str = "jarvis-models-v1";
```

### 4. Progressive Loading

Load models on-demand rather than upfront to improve initial load time.

## Environment Variables

JARVIS doesn't require runtime environment variables, but you may want to configure:

### Build-time

Set in `Trunk.toml` or pass to trunk:

```toml
[build]
public_url = "/my-app/"  # For subdirectory hosting
```

## Monitoring and Analytics

### Add Analytics

Insert analytics in `index.html`:

```html
<!-- Plausible Analytics -->
<script defer data-domain="yourdomain.com" 
        src="https://plausible.io/js/script.js"></script>
```

### Error Tracking

Integrate Sentry or similar:

```rust
// In lib.rs
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    
    // Add error reporting here
    // sentry_init(...);
    
    leptos::mount::mount_to_body(App);
}
```

## Security Considerations

1. **HTTPS Required**: Always use HTTPS in production
2. **CORS Headers**: Set appropriate CORS headers for MCP server communication
3. **Content Security Policy**: Add CSP headers for additional security
4. **Model Sources**: Only load models from trusted sources

### Example CSP Header

```
Content-Security-Policy: default-src 'self'; 
  script-src 'self' 'unsafe-inline' 'unsafe-eval'; 
  style-src 'self' 'unsafe-inline' https://cdn.tailwindcss.com; 
  worker-src 'self' blob:;
  connect-src 'self' https://huggingface.co;
```

## Troubleshooting

### WASM Module Not Loading

- Check MIME type is set to `application/wasm`
- Verify files are served over HTTPS
- Check browser console for errors

### Performance Issues

- Enable compression (gzip/brotli)
- Use a CDN
- Implement model caching
- Profile with browser DevTools

### CORS Errors with MCP

- Add appropriate CORS headers
- Use a proxy for MCP servers if needed
- Check MCP server configuration

## Continuous Deployment

### GitHub Actions Example

```yaml
name: Deploy

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      - run: cargo install trunk
      - run: trunk build --release
      - uses: your-deployment-action@v1
        with:
          directory: ./dist
```

## Next Steps

After deployment:

1. Test on multiple browsers
2. Monitor performance and errors
3. Set up analytics
4. Configure CDN caching
5. Enable HTTPS
6. Test MCP server integration

## Support

For deployment issues, please open an issue on GitHub or check the [troubleshooting guide](README.md#troubleshooting).
