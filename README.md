# jtp - for moving data more efficiently

## Why
- send and retrieve data using the ubiquitous JSON format
- strip down on the faff
- send only vital information
- updated for the new web

## Format
### GET
```json
{
    "jtp": {
        "version": "0.1.0",
        "type": "get" // therefore sent from a user
    },

    "host": {
        "os": {
            "host-triple": "x86_64-unknown-linux-gnu",
            "distro": "debian-12",
        },
        "net": {
            "ipv4": "1.2.3.4",
            "ipv6": "2001:0db8:85a3:0000:0000:0000:0000:0000"
        },
        "agent": "SampleImpl/1.0" // sender name and version
    },

    "payload": {
        "head": {
            "url": "https://example.com/jtp",
        },

        "body": { // like headers in HTTP
            "example-field": "example-value",
            "supports-nesting": {
                "yes": true,
                "no": false
            }
        }
    }
}
```

### ACK
```json
{
   "jtp": {
        "version": "0.1.0",
        "type": "ack" // therefore coming from a server
    }, 

    "host": {
        "os": {
            "host-triple": "x86_64-windows-msvc",
            "distro": "windows-10-22h2"
        },
        "net": {
            "ipv4": "1.2.3.5",
            "ipv6": "2001:0db8:85a3:0000:0000:0000:0000:0005"
        }
    },

    "payload": {
        "head": {
            "date": "2024-01-16 11:30:00 +0000",
            "status": 200, // HTTP error code
            "url": "https://example.com/jtp"
        },
        "body": "Hello from https://example.com/jtp!"
    }
}
```