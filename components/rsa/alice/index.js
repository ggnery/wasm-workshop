import net from 'net'
import { send_encrypted_message } from "./network.js"
import { getPublicKey, encrypt, decrypt } from './crypto/crypto.js';

const HOST = "127.0.0.1"
const PORT = "8080"
const client = new net.Socket();

let BOB_PUBLIC_KEY

//Connect to bob
client.connect(PORT, HOST, () =>{
    console.log("Connected to bob in " + HOST + ":" + PORT + "\n")
})

//Recieve Bob public key
client.on("data", (data) => {
    BOB_PUBLIC_KEY = data.toString("utf8")
    process.stdout.write("======================================\n")
    process.stdout.write("BOB PUBLIC KEY\n")
    process.stdout.write("======================================\n")
    process.stdout.write(BOB_PUBLIC_KEY)
    process.stdout.write("======================================\n")

    process.stdout.write("Type a message for Bob: ")
})

// Read data from input
process.stdin.on("data", (message) => {
    let encrypted_message = encrypt(message.toString("utf8"), BOB_PUBLIC_KEY)
    send_encrypted_message(encrypted_message, client)
    process.stdout.write("Type a message for Bob: ")
})    