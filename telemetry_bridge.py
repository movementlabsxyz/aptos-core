#!/usr/bin/env python3
import http.server
import socketserver
import requests
import gzip
import json
import sys
from urllib.parse import urlparse, parse_qs

class TelemetryBridge(http.server.BaseHTTPRequestHandler):
    def log_message(self, format, *args):
        # Custom logging to see what's happening
        print(f"[{self.address_string()}] {format % args}")
    
    def do_GET(self):
        print(f"GET request to: {self.path}")
        
        if self.path == '/api/v1/':
            # Return server info for initial handshake
            # Generate a fake but properly formatted x25519 public key (32 bytes = 64 hex chars)
            fake_public_key = "a" * 64  # 32 bytes of 0xaa in hex format
            response = {
                "public_key": fake_public_key
            }
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps(response).encode())
            print(f"Sent server info response with public key: {fake_public_key}")
            
        elif self.path.startswith('/api/v1/chain-access/'):
            # Extract chain ID from path
            chain_id = self.path.split('/')[-1]
            print(f"Chain access check for chain ID: {chain_id}")
            
            # Always return true to indicate the service supports this chain
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            self.wfile.write(b'true')
            print(f"Approved chain access for chain ID: {chain_id}")
            
        else:
            print(f"Unknown GET endpoint: {self.path}")
            self.send_response(200)
            self.end_headers()
            self.wfile.write(b'OK')
    
    def do_POST(self):
        print(f"POST request to: {self.path}")
        
        if self.path.startswith('/api/v1/ingest/metrics'):
            print("Received metrics data!")
            
            # Read the request body
            content_length = int(self.headers.get('Content-Length', 0))
            post_data = self.rfile.read(content_length)
            print(f"Received {len(post_data)} bytes of data")
            
            # Check if data is gzipped
            if self.headers.get('Content-Encoding') == 'gzip':
                print("Data is gzipped, decompressing...")
                try:
                    post_data = gzip.decompress(post_data)
                    print(f"Decompressed to {len(post_data)} bytes")
                except Exception as e:
                    print(f"Failed to decompress: {e}")
                    self.send_response(500)
                    self.end_headers()
                    return
            
            # Show a sample of the metrics data
            data_sample = post_data[:500].decode('utf-8', errors='ignore')
            print(f"Metrics sample: {data_sample}...")
            
            # Forward to push gateway
            try:
                print("Forwarding to push gateway...")
                response = requests.post(
                    'http://localhost:9091/metrics/job/aptos_validator',
                    data=post_data,
                    headers={'Content-Type': 'text/plain'},
                    timeout=10
                )
                print(f"Push gateway response: {response.status_code}")
                
                if response.status_code == 200:
                    print("Successfully forwarded metrics to push gateway!")
                else:
                    print(f"Push gateway error: {response.text}")
                
                self.send_response(200)
                self.send_header('Content-Type', 'application/json')
                self.end_headers()
                self.wfile.write(b'{"status": "ok"}')
                
            except Exception as e:
                print(f"Error forwarding to push gateway: {e}")
                self.send_response(500)
                self.end_headers()
        
        elif self.path.startswith('/api/v1/auth'):
            print("Authentication request received")
            
            # Read the auth request to see what the node is sending
            content_length = int(self.headers.get('Content-Length', 0))
            auth_request_data = self.rfile.read(content_length)
            print(f"Auth request data: {auth_request_data[:200]}...")
            
            # The noise protocol authentication is too complex to fake properly.
            # Instead, let's return an error that might cause the client to skip auth
            # or use a different approach
            
            print("Returning auth failure to see if client falls back to different method")
            self.send_response(401)  # Unauthorized
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            error_response = {
                "code": 401,
                "message": "Authentication not supported in bridge mode"
            }
            self.wfile.write(json.dumps(error_response).encode())
        
        elif self.path.startswith('/api/v1/ingest/custom-event'):
            print("Custom event received (ignoring)")
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            self.wfile.write(b'{"status": "ok"}')
        
        elif self.path.startswith('/api/v1/ingest/logs'):
            print("Log data received (ignoring)")
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            self.wfile.write(b'{"status": "ok"}')
        
        else:
            print(f"Unknown POST endpoint: {self.path}")
            self.send_response(200)
            self.end_headers()
            self.wfile.write(b'{"status": "ok"}')

if __name__ == "__main__":
    PORT = 8011
    print(f"Starting Enhanced Telemetry-to-PushGateway bridge on port {PORT}")
    print("This bridge will:")
    print("1. Handle chain access checks")
    print("2. Handle authentication requests") 
    print("3. Forward prometheus metrics to push gateway")
    print("4. Provide detailed logging")
    print()
    print("Make sure your push gateway is running on localhost:9091")
    print()
    
    try:
        # Enable socket reuse to prevent "Address already in use" errors
        socketserver.TCPServer.allow_reuse_address = True
        with socketserver.TCPServer(("", PORT), TelemetryBridge) as httpd:
            print(f"Bridge is ready! Listening on http://localhost:{PORT}")
            httpd.serve_forever()
    except KeyboardInterrupt:
        print("\nShutting down bridge...")
    except Exception as e:
        print(f"Error starting bridge: {e}")
        sys.exit(1)
