

def recive_public_key(sock):
    pub_key_len_bytes = sock.recv(8)
    if len(pub_key_len_bytes) < 8:
            raise Exception("Failed to recieve public key size")
    pub_key_len = int.from_bytes(pub_key_len_bytes, byteorder='big')
    
    data = b''
    while len(data) < pub_key_len:
        packet = sock.recv(pub_key_len - len(data))
        if not packet:
            raise Exception("Connection finished before recieving all data")
        data += packet

    
    public_key = data.decode('utf-8')
    return public_key
    