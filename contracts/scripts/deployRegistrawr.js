const { ethers } = require("hardhat");

async function main() {
  const Dapp = await ethers.getContractFactory("Dapps");
  const dapp = await Dapp.deploy();

  const Registrawr = await ethers.getContractFactory("Registrawr", {
    libraries: {
      Dapps: dapp.address,
    },
  });
  const registrawr = await Registrawr.deploy();

  console.log("Registrawr deployed to:", registrawr.address);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
