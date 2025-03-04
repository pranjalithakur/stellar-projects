// SPDX-License-Identifier: Apache-2.0
pragma solidity >=0.8.0;
pragma abicoder v2;

import "@iconfoundation/xcall-solidity-library/utils/RLPDecode.sol";
import "./Types.sol";

library RLPDecodeStruct {
    using RLPDecode for RLPDecode.RLPItem;
    using RLPDecode for RLPDecode.Iterator;
    using RLPDecode for bytes;

    using RLPDecodeStruct for bytes;

    uint8 private constant LIST_SHORT_START = 0xc0;
    uint8 private constant LIST_LONG_START = 0xf7;

    function decodeCSMessage(
        bytes memory _rlp
    ) internal pure returns (Types.CSMessage memory) {
        RLPDecode.RLPItem[] memory ls = _rlp.toRlpItem().toList();
        return
            Types.CSMessage(
                ls[0].toInt(),
                ls[1].toBytes() //  bytes array of RLPEncode(Data)
            );
    }

    function decodeCSMessageResponse(bytes memory _rlp)
        internal
        pure
    returns (Types.CSMessageResponse memory)
    {
        RLPDecode.RLPItem[] memory ls = _rlp.toRlpItem().toList();
        return
        Types.CSMessageResponse(
            ls[0].toUint(),
            int(ls[1].toInt())
        );
    }

    function toStringArray(
        RLPDecode.RLPItem memory item
    ) internal pure returns (string[] memory) {
        RLPDecode.RLPItem[] memory ls = item.toList();
        string[] memory protocols = new string[](ls.length);
        for (uint256 i = 0; i < ls.length; i++) {
            protocols[i] = string(ls[i].toBytes());
        }
        return protocols;
    }

    function decodeCSMessageRequest(bytes memory _rlp)
        internal
        pure
    returns (Types.CSMessageRequest memory)
    {
        RLPDecode.RLPItem[] memory ls = _rlp.toRlpItem().toList();
        return
        Types.CSMessageRequest(
            string(ls[0].toBytes()),
            string(ls[1].toBytes()),
            ls[2].toUint(),
            ls[3].toBoolean(),
            ls[4].toBytes(),
            toStringArray(ls[5])
        );
    }

    function decodeCSMessageRequestV2(
        bytes memory _rlp
    ) internal pure returns (Types.CSMessageRequestV2 memory) {
        RLPDecode.RLPItem[] memory ls = _rlp.toRlpItem().toList();
        return
            Types.CSMessageRequestV2(
                string(ls[0].toBytes()),
                string(ls[1].toBytes()),
                ls[2].toUint(),
                ls[3].toInt(),
                ls[4].toBytes(),
                toStringArray(ls[5])
            );
    }

    function decodeCallMessageWithRollback(
        bytes memory _rlp
    ) internal pure returns (Types.CallMessageWithRollback memory) {
        RLPDecode.RLPItem[] memory ls = _rlp.toRlpItem().toList();
        return
            Types.CallMessageWithRollback(
                ls[0].toBytes(),
                ls[1].toBytes()
            );
    }


    function decodeXCallEnvelope(
        bytes memory _rlp
    ) internal pure returns (Types.XCallEnvelope memory) {
        RLPDecode.RLPItem[] memory ls = _rlp.toRlpItem().toList();
        
        return
            Types.XCallEnvelope(
                ls[0].toInt(),
                ls[1].toBytes(),
                toStringArray(ls[2]),
                toStringArray(ls[3])
            );
    }

    function decodeCSMessageResult(
        bytes memory _rlp
    ) internal pure returns (Types.CSMessageResult memory) {
        RLPDecode.RLPItem[] memory ls = _rlp.toRlpItem().toList();
        return Types.CSMessageResult(ls[0].toUint(), ls[1].toInt(), ls[2].toBytes());
    }
}
