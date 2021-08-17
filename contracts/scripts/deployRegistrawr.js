const { ethers } = require("hardhat");

async function main() {
  const Registrawr = await ethers.getContractFactory("Registrawr");
  const registrawr = await Registrawr.deploy();

  console.log("Registrawr deployed to:", registrawr.address);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
