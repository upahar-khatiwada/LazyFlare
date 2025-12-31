# Rust DNS Server

A simple DNS server written in Rust. It supports **A** and **CNAME** record queries and sends proper DNS responses back to the client.

---

## Features

- Handles **Type A** requests
- Handles **Type CNAME** requests
- Properly constructs DNS response

---

## Zone File Setup

Place your zone files as `.txt` files inside the `records/` folder. Make sure each file is named after the **type A domain** it represents.

Each zone file should start with **26 lines of comments**, similar to Cloudflare style:

```
; Zone file for example.com
; Managed by Rust DNS Server
;
; Please follow the format:
; TYPE NAME VALUE
;
; A record example:
; A www 192.168.1.1
; CNAME record example:
; CNAME blog www
;
; TTL can be added optionally
; Default TTL: 3600
; Additional comments...
; Comment line 12
; Comment line 13
; Comment line 14
; Comment line 15
; Comment line 16
; Comment line 17
; Comment line 18
; Comment line 19
; Comment line 20
; Comment line 21
; Comment line 22
; Comment line 23
; Comment line 24
; Comment line 25
; Comment line 26
```

After the comments, you can add your records:

```
upaharkhatiwada.com.np 3600 IN SOA khloe.ns.cloudflare.com. dns.cloudflare.com. 2051868458 10000 2400 604800 3600
;; NS Records
upaharkhatiwada.com.np. 86400 IN NS khloe.ns.cloudflare.com.
upaharkhatiwada.com.np. 86400 IN NS west.ns.cloudflare.com.
```

---

## Example Output

**Type A Response:**

![Type A Response](https://github.com/user-attachments/assets/fcbdab8d-3981-4fb9-ae74-1da721f39dda)

**Type CNAME Response:**

![Type CNAME Response](https://github.com/user-attachments/assets/c41b62c2-d768-48da-b348-37b600b796e4)


---

## Usage

1. Place your zone file in the `records/` folder.  
2. Run the server:

```bash
cargo run
```

3. Send DNS queries using any DNS client. The server will respond to **A** and **CNAME** queries accordingly.

---

## Notes

- Only **A** and **CNAME** records are supported.  
- Make sure the zone file follows the format correctly to avoid parsing errors.
