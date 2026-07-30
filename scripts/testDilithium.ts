import { ethers } from "hardhat";

async function main() { 
console.log("🚀 Deploying DilithiumVerifier contract..."); 

const Verifier = await ethers.getContractFactory("DilithiumVerifier"); 
const verifier = await Verifier.deploy(); 

await verifier.deployed(); 
console.log("✅ DilithiumVerifier deployed at:", verifier.address); 

// Dummy values ​​for test 
const message = ethers.utils.toUtf8Bytes("hello coiwin PQC"); 
const signature = ethers.utils.hexlify(ethers.utils.randomBytes(32)); 
const pubkey = ethers.utils.hexlify(ethers.utils.randomBytes(32)); 

console.log("🔍 Calling verifyDilithium..."); 
const tx = await verifier.verifyDilithium(message, signature, pubkey); 
console.log("✅ verifyDilithium result:", tx);
}

main().catch((error) => { 
console.error(error); 
process.exitCode = 1;
});
