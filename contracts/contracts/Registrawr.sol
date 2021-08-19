//SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "hardhat/console.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract Registrawr is ERC721URIStorage {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;
    mapping(uint256 => string) private _dapps;

    constructor() ERC721("Registration", "RAWR") {}

    function register(string memory name, string memory tokenURI)
        public
        returns (uint256)
    {
        console.log(
            "Registering new dapp with name %s and tokenURI %s",
            name,
            tokenURI
        );
        _tokenIds.increment();

        uint256 newRegId = _tokenIds.current();
        _mint(msg.sender, newRegId);
        _setTokenURI(newRegId, tokenURI);

        _dapps[newRegId] = name;
        return newRegId;
    }

    function listDapps() public view returns (string[] memory) {
        uint256 numTokenIds = _tokenIds.current();
        console.log("Listing %s dapps", numTokenIds);
        if (numTokenIds == 0) {
            string[] memory dappNames = new string[](1);
            dappNames[0] = "";

            return dappNames;
        } else {
            console.log("Num token ids: %s", numTokenIds);
            string[] memory dappNames = new string[](numTokenIds);

            for (uint256 i = 0; i < numTokenIds; i++) {
                console.log("index %s", i);
                console.log("Adding dapp %s", _dapps[i + 1]);
                dappNames[i] = _dapps[i + 1];
            }

            return dappNames;
        }
    }
}
