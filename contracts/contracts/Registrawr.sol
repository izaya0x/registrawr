//SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "hardhat/console.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/utils/Counters.sol";
import "./Dapp.sol";

// TODO: Add extra data to make sure that the code is not tampered with
// - Hash the built code itself
// - Record the git commit hash that this was uploaded from
contract Registrawr is ERC721URIStorage {
    using Counters for Counters.Counter;
    using Dapps for Dapps.Dapp;

    Counters.Counter private _tokenIds;
    Dapps.Dapp private _dapps;

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

        _dapps.set(name, newRegId);
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
            string[] memory dappNames = new string[](_dapps.size());

            for (uint256 i = 0; i < _dapps.size(); i++) {
                console.log("index %s", i);
                console.log("Adding dapp %s", _dapps.getKeyAtIndex(i));
                dappNames[i] = _dapps.getKeyAtIndex(i);
            }

            return dappNames;
        }
    }

    function getDapp(string memory name) public view returns (string memory) {
        uint256 regId = _dapps.get(name);
        return tokenURI(regId);
    }
}
