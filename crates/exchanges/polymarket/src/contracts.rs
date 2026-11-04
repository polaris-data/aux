//! Client-facing contract interfaces pinned to the official unified SDK.
use alloy_sol_types::sol;
sol! {
 interface IERC20 {
  function approve(address spender, uint256 amount);
  function allowance(address owner, address spender) view returns (uint256);
  function transfer(address recipient, uint256 amount);
 }
}
sol! {
 interface IDepositWallet {
  function authorizeSessionSigner(address sessionSigner, uint256 validUntil);
  function revokeSessionSigner(address sessionSigner);
 }
}
sol! {
 interface IPerpsDeposit {
  function deposit(address token, uint256 amount, address to);
 }
}
sol! {
 interface IPositions {
  function setApprovalForAll(address operator, bool approved);
  function isApprovedForAll(address account, address operator) view returns (bool);
  function balanceOf(address account, uint256 id) view returns (uint256);
  function balanceOfBatch(address[] accounts, uint256[] ids) view returns (uint256[]);
 }
}
sol! {
 interface ICtf {
  function splitPosition(address collateralToken, bytes32 parentCollectionId, bytes32 conditionId, uint256[] partition, uint256 amount);
  function mergePositions(address collateralToken, bytes32 parentCollectionId, bytes32 conditionId, uint256[] partition, uint256 amount);
  function redeemPositions(address collateralToken, bytes32 parentCollectionId, bytes32 conditionId, uint256[] indexSets);
 }
}
sol! {
 interface IRouterV2 {
  function split(bytes31 conditionId, uint256 amount);
  function convert(bytes29 eventId, uint16 conditionIndex, uint256 amount);
  function horizontalSplit(bytes29 eventId, uint256 amount);
  function horizontalMerge(bytes29 eventId, uint256 amount);
  function merge(bytes31 conditionId, uint256 amount);
  function redeem(bytes31 conditionId, uint256 outcomeIndex, uint256 amount);
 }
}
sol! {
 interface ICombinatorialModule {
  function prepareCondition(uint256[] legs) returns (bytes31);
 }
}
sol! {
 interface IMultisend {
  function multiSend(bytes);
 }
}
sol! {
 interface IProxyFactory {
struct Call { uint8 typeCode; address to; uint256 value; bytes data; }
  function proxy(Call[] calls) returns (bytes[]);
 }
}
