function send_encrypted_message(encrypted_message, client) {
    // Send the size of encrypted message buffer
    const buffer = new ArrayBuffer(4);
    const view = new DataView(buffer);
    view.setInt32(0, encrypted_message.length, false);
    const byteArray = new Uint8Array(buffer);
    client.write(byteArray)
    
    //Send encrypted message buffer
    client.write(encrypted_message)
}

export { send_encrypted_message }