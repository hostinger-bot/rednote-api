## Rednote (Xiaohongshu / 小红书) Unofficial API

Fast, free, and open-source Rednote (小红书) video/image downloader built with Rust + Axum.
<p align="center">
  <a href="https://github.com/hostinger-bot/rednote-api/actions/workflows/rust.yml">
    <img src="https://github.com/hostinger-bot/rednote-api/actions/workflows/rust.yml/badge.svg" alt="Rust" />
  </a>
</p>

<p align="center">
  <a href="https://github.com/hostinger-bot/rednote-api/actions/workflows/rust-clippy.yml">
    <img src="https://github.com/hostinger-bot/rednote-api/actions/workflows/rust-clippy.yml/badge.svg" alt="rust-clippy" />
  </a>
</p>


---

## 🚀 About This Project

This is an unofficial, reverse-engineered API for downloading Rednote (Xiaohongshu) media.  
It extracts:
- Post information
- High-resolution images
- Original video URLs (no watermark)
- Title, keywords, description
- Engagement data (likes, comments, collects)

Fully open-source and optimized for performance.

---

## ✨ Features

- Download Xiaohongshu videos without watermark  
- Extract all post images  
- Built with async Rust  
- Supports GET and POST  
- CORS enabled (browser-friendly)  
- Complete OpenAPI 3.0 documentation  
- Every response includes `status: true/false`  

---

## 📦 Installation

```sh
git clone https://github.com/hostinger-bot/rednote-api.git
cd rednote-api
cargo build
cargo build --release
```

### Start the Server

```sh
cargo run
# or for production
cargo run --release
```

Default URL: http://localhost:4000

### Development Commands

```sh
cargo check
cargo fmt
cargo clippy
```

### Testing

```sh
cargo test
```

---

## 🔥 API Endpoints

### GET /api/rednote

Query Parameters:
- `url` (string, required)

Example:
```
/api/rednote?url=http://xhslink.com/o/21DKXV988zp
```

### POST /api/rednote

Body:

```json
{
  "url": "http://xhslink.com/o/21DKXV988zp"
}
```

---

## 📤 Success Response Example

```json
{
  "desc": "这碗面不仅是食物，更是快乐的来源！好吃到想每天吃!",
  "downloads": [],
  "duration": "",
  "engagement": {
    "collects": "0",
    "comments": "2",
    "likes": "1"
  },
  "images": [
    "http://sns-webpic-qc.xhscdn.com/202601081924/6bd0dbeff2d295034320d652e5920a11/1040g00831ej7ki91h26g5ps768227md5hqjjk70!nd_dft_wlteh_jpg_3"
  ],
  "keywords": "",
  "nickname": "这碗面不仅是食物，更是快乐的来源！好吃到想每天吃!",
  "noteId": "67c6308c000000002a00fff2",
  "status": true,
  "title": "这碗面不仅是食物，更是快乐的来源！好吃到想每天吃! - 小红书"
}
```

---

## ❌ Error Response Example

```json
{
  "status": false,
  "error": "Invalid Xiaohongshu URL"
}
```

---

## 📚 API Documentation

- GET `/openapi.json`
- GET `/docs`

---

## 🧩 Response Schema

| Field | Type | Description |
|-------|------|-------------|
| noteId | string | Unique Rednote post ID |
| nickname | string | Author name |
| title | string | Post title |
| desc | string | Description |
| keywords | string | Extracted keywords |
| duration | string | Video duration |
| engagement | object | Likes, comments, collects |
| images | string[] | List of image URLs |
| downloads | object[] | Video download sources |
| status | bool | API success flag |
| error | string | Error message (only on failure) |

---

## 🤝 Contributing

PRs are welcome. Open issues, submit fixes, or suggest new features.

---

## 📄 License

MIT License © 2025 Tio (BOTCAHX)

---

## ⭐ Support

Give this project a ★ on [GitHub](https://github.com/hostinger-bot/rednote-api).
