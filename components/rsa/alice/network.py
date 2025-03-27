

def recive_public_key(sock):
    pub_key_len_bytes = sock.recv(8)
    if len(pub_key_len_bytes) < 8:
            raise Exception("Failed to recieve public key size")
    pub_key_len = int.from_bytes(pub_key_len_bytes, byteorder='big')
    
    data = sock.recv(pub_key_len)
    
    public_key = data.decode('utf-8')
    print(public_key)
    return public_key
    