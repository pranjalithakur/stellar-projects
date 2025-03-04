// SPDX-License-Identifier: Apache-2.0
pragma solidity >=0.8.0;
pragma abicoder v2;

import "@iconfoundation/xcall-solidity-library/utils/RLPEncode.sol";
import "./Types.sol";

library RLPEncodeStruct {
    using RLPEncode for bytes;
    using RLPEncode for string;
    using RLPEncode for uint256;
    using RLPEncode for int256;
    using RLPEncode for address;
    using RLPEncode for bool;

    using RLPEncodeStruct for Types.CSMessage;
    using RLPEncodeStruct for Types.CSMessageRequest;
    using RLPEncodeStruct for Types.CSMessageResult;

    function encodeCSMessage(
        Types.CSMessage memory _bs
    ) internal pure returns (bytes memory) {
        bytes memory _rlp = abi.encodePacked(
            _bs.msgType.encodeInt(),
            _bs.payload.encodeBytes()
        );
        return _rlp.encodeList();
    }

    function encodeCSMessageRequest(Types.CSMessageRequest memory _bs)
        internal
        pure
        returns (bytes memory)
    {

        bytes memory _protocols;
        bytes memory temp;
        for (uint256 i = 0; i < _bs.protocols.length; i++) {
            temp = abi.encodePacked(_bs.protocols[i].encodeString());
            _protocols = abi.encodePacked(_protocols, temp);
        }
        bytes memory _rlp =
            abi.encodePacked(
                _bs.from.encodeString(),
                _bs.to.encodeString(),
                _bs.sn.encodeUint(),
                _bs.rollback.encodeBool(),
                _bs.data.encodeBytes(),
                _protocols.encodeList()

            );
        return _rlp.encodeList();
    }

    function encodeCSMessageRequestV2(
        Types.CSMessageRequestV2 memory _bs
    ) internal pure returns (bytes memory) {
        bytes memory _protocols;
        bytes memory temp;
        for (uint256 i = 0; i < _bs.protocols.length; i++) {
            temp = abi.encodePacked(_bs.protocols[i].encodeString());
            _protocols = abi.encodePacked(_protocols, temp);
        }
        bytes memory _rlp = abi.encodePacked(
            _bs.from.encodeString(),
            _bs.to.encodeString(),
            _bs.sn.encodeUint(),
            _bs.messageType.encodeInt(),
            _bs.data.encodeBytes(),
            _protocols.encodeList()
        );
        return _rlp.encodeList();
    }

    function encodeCSMessageResponse(Types.CSMessageResponse memory _bs)
        internal
        pure
        returns (bytes memory)
    {
        bytes memory _rlp =
            abi.encodePacked(
                _bs.sn.encodeUint(),
                _bs.code.encodeInt()
            );
        return _rlp.encodeList();
    }

    function encodeXCallEnvelope(
        Types.XCallEnvelope memory env
    ) internal pure returns (bytes memory) {

        bytes memory _sources;
        bytes memory temp;

        for (uint256 i = 0; i < env.sources.length; i++) {
            temp = abi.encodePacked(env.sources[i].encodeString());
            _sources = abi.encodePacked(_sources, temp);
        }

        bytes memory _dests;
        for (uint256 i = 0; i < env.destinations.length; i++) {
            temp = abi.encodePacked(env.destinations[i].encodeString());
            _dests = abi.encodePacked(_dests, temp);
        }

        bytes memory _rlp = abi.encodePacked(
            env.messageType.encodeInt(),
            env.message.encodeBytes(),
            _sources.encodeList(),
            _dests.encodeList()
        );

        return _rlp.encodeList();
    }

    function encodeCSMessageResult(
        Types.CSMessageResult memory _bs
    ) internal pure returns (bytes memory) {
        bytes memory _rlp = abi.encodePacked(
            _bs.sn.encodeUint(),
            _bs.code.encodeInt(),
            _bs.message.encodeBytes()
        );
        return _rlp.encodeList();
    }

    function encodeCallMessageWithRollback(
        Types.CallMessageWithRollback memory _bs
    ) internal pure returns (bytes memory) {
        bytes memory _rlp = abi.encodePacked(
            _bs.data.encodeBytes(),
            _bs.rollback.encodeBytes()
        );
        return _rlp.encodeList();
    }
}
