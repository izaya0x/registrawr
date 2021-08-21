// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

library Dapps {
    struct Dapp {
        string[] names;
        mapping(string => uint256) tokenId;
        mapping(string => uint256) indexOf;
        mapping(string => bool) inserted;
    }

    function get(Dapp storage dapp, string memory name)
        public
        view
        returns (uint256)
    {
        return dapp.tokenId[name];
    }

    function getKeyAtIndex(Dapp storage dapp, uint256 index)
        public
        view
        returns (string memory)
    {
        return dapp.names[index];
    }

    function size(Dapp storage dapp) public view returns (uint256) {
        return dapp.names.length;
    }

    function set(
        Dapp storage dapp,
        string memory name,
        uint256 tokenId
    ) public {
        if (dapp.inserted[name]) {
            dapp.tokenId[name] = tokenId;
        } else {
            dapp.inserted[name] = true;
            dapp.tokenId[name] = tokenId;
            dapp.indexOf[name] = dapp.names.length;
            dapp.names.push(name);
        }
    }

    function remove(Dapp storage dapp, string memory name) public {
        if (!dapp.inserted[name]) {
            return;
        }

        // Delete name entry from inserted and tokenId
        delete dapp.inserted[name];
        delete dapp.tokenId[name];

        // Move the name that is in the last index position to the
        // index of the name we're about to delete
        uint256 index = dapp.indexOf[name];
        uint256 lastIndex = dapp.names.length - 1;
        string memory lastName = dapp.names[lastIndex];

        dapp.indexOf[lastName] = index;
        delete dapp.indexOf[name];

        //Set the name that was in the last index position to the deleted
        // name's posotion. And pop the last value.
        dapp.names[index] = lastName;
        dapp.names.pop();
    }
}
