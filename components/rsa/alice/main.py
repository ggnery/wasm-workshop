import socket
from network import recive_public_key

HOST = '127.0.0.1'
PORT = 8080

def main():
    with socket.create_connection((HOST, PORT)) as sock:
        print(f"Connected to server in {HOST}:{PORT}")
 
        public_key = recive_public_key(sock)
        

        
if __name__ == "__main__":
    main()    