#![allow(deprecated)]
// Adapted from Rise's pinned cross-language instruction compatibility example.
use aux_rs_phoenix::instructions::prelude::*;
use solana_pubkey::Pubkey;
use std::collections::BTreeMap;
#[test]
fn reference_instruction_bytes() {
    let mut results = BTreeMap::new();
    let mut dispatch = BTreeMap::new();
    // Define 20 pubkey constants: [0u8; 31] + [N] for N=0..19
    let pubkeys: Vec<Pubkey> = (0..20)
        .map(|i| {
            let mut bytes = [0u8; 32];
            bytes[31] = i;
            Pubkey::from(bytes)
        })
        .collect();

    // Helper for array of 2 pubkeys
    let vec2 = |a: usize, b: usize| vec![pubkeys[a], pubkeys[b]];

    {
        let params = LimitOrderParams::builder()
            .trader(pubkeys[0])
            .trader_account(pubkeys[1])
            .perp_asset_map(pubkeys[2])
            .orderbook(pubkeys[3])
            .spline_collection(pubkeys[4])
            .global_trader_index(vec2(5, 6))
            .active_trader_buffer(vec2(7, 8))
            .side(Side::Bid)
            .price_in_ticks(1000)
            .num_base_lots(100)
            .build()
            .unwrap();

        let ix = create_place_limit_order_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_place_limit_order_ix", request.clone())
                .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_place_limit_order_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("PlaceLimitOrder".to_string(), hex_encode(&ix.data));
    }
    {
        let params = MarketOrderParams::builder()
            .trader(pubkeys[0])
            .trader_account(pubkeys[1])
            .perp_asset_map(pubkeys[2])
            .orderbook(pubkeys[3])
            .spline_collection(pubkeys[4])
            .global_trader_index(vec2(5, 6))
            .active_trader_buffer(vec2(7, 8))
            .side(Side::Ask)
            .num_base_lots(100)
            .min_base_lots_to_fill(0)
            .min_quote_lots_to_fill(0)
            .self_trade_behavior(SelfTradeBehavior::Abort)
            .client_order_id(0)
            .order_flags(OrderFlags::None)
            .cancel_existing(false)
            .build()
            .unwrap();

        let ix = create_place_market_order_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_place_market_order_ix", request.clone())
                .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_place_market_order_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("PlaceMarketOrder".to_string(), hex_encode(&ix.data));
    }
    {
        let params = MultiLimitOrderParams::builder()
            .trader(pubkeys[0])
            .trader_account(pubkeys[1])
            .perp_asset_map(pubkeys[2])
            .orderbook(pubkeys[3])
            .spline_collection(pubkeys[4])
            .global_trader_index(vec2(5, 6))
            .active_trader_buffer(vec2(7, 8))
            .build()
            .unwrap();

        let ix = create_place_multi_limit_order_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_place_multi_limit_order_ix", request.clone())
                .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_place_multi_limit_order_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("PlaceMultiLimitOrder".to_string(), hex_encode(&ix.data));
    }
    {
        let params = CancelOrdersByIdParams::builder()
            .trader(pubkeys[0])
            .trader_account(pubkeys[1])
            .perp_asset_map(pubkeys[2])
            .orderbook(pubkeys[3])
            .spline_collection(pubkeys[4])
            .global_trader_index(vec2(5, 6))
            .active_trader_buffer(vec2(7, 8))
            .order_ids(vec![CancelId::new(1000, 0)])
            .build()
            .unwrap();

        let ix = create_cancel_orders_by_id_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_cancel_orders_by_id_ix", request.clone())
                .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_cancel_orders_by_id_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("CancelOrdersById".to_string(), hex_encode(&ix.data));
    }
    {
        let params = CancelAllParams::builder()
            .trader(pubkeys[0])
            .trader_account(pubkeys[1])
            .perp_asset_map(pubkeys[2])
            .orderbook(pubkeys[3])
            .spline_collection(pubkeys[4])
            .global_trader_index(vec2(5, 6))
            .active_trader_buffer(vec2(7, 8))
            .build()
            .unwrap();

        let ix = create_cancel_all_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_cancel_all_ix", request.clone()).unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_cancel_all_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("CancelAll".to_string(), hex_encode(&ix.data));
    }
    {
        let params = CancelUpToParams::builder()
            .trader(pubkeys[0])
            .trader_account(pubkeys[1])
            .perp_asset_map(pubkeys[2])
            .orderbook(pubkeys[3])
            .spline_collection(pubkeys[4])
            .global_trader_index(vec2(5, 6))
            .active_trader_buffer(vec2(7, 8))
            .side(Side::Ask)
            .num_orders_to_cancel(2)
            .tick_limit(1000)
            .build()
            .unwrap();

        let ix = create_cancel_up_to_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_cancel_up_to_ix", request.clone()).unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_cancel_up_to_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("CancelUpTo".to_string(), hex_encode(&ix.data));
    }
    {
        let params = UncrossCrankParams::builder()
            .perp_asset_map(pubkeys[2])
            .orderbook(pubkeys[3])
            .spline_collection(pubkeys[4])
            .global_trader_index(vec2(5, 6))
            .active_trader_buffer(vec2(7, 8))
            .match_limit(5)
            .build()
            .unwrap();

        let ix = create_uncross_crank_ix(params).unwrap();
        results.insert("UncrossCrank".to_string(), hex_encode(&ix.data));
    }
    {
        let params = CancelStopLossParams::builder()
            .funder(pubkeys[0])
            .trader_account(pubkeys[1])
            .position_authority(pubkeys[2])
            .asset_id(0)
            .execution_direction(Direction::GreaterThan)
            .build()
            .unwrap();

        let ix = create_cancel_stop_loss_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_cancel_stop_loss_ix", request.clone())
                .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_cancel_stop_loss_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("CancelStopLoss".to_string(), hex_encode(&ix.data));
    }
    {
        let params = PlaceStopLossParams::builder()
            .funder(pubkeys[0])
            .trader_account(pubkeys[1])
            .position_authority(pubkeys[2])
            .perp_asset_map(pubkeys[3])
            .orderbook(pubkeys[4])
            .spline_collection(pubkeys[5])
            .global_trader_index(vec2(6, 7))
            .active_trader_buffer(vec2(8, 9))
            .stop_loss_account(pubkeys[10])
            .asset_id(0)
            .trigger_price(1000)
            .execution_price(999)
            .trade_side(Side::Ask)
            .execution_direction(Direction::LessThan)
            .order_kind(StopLossOrderKind::IOC)
            .build()
            .unwrap();

        let ix = create_place_stop_loss_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_place_stop_loss_ix", request.clone())
                .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_place_stop_loss_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("PlaceStopLoss".to_string(), hex_encode(&ix.data));
    }
    {
        let params = CreateConditionalOrdersAccountParams::builder()
            .payer(pubkeys[0])
            .trader_wallet(pubkeys[1])
            .trader_account(pubkeys[2])
            .trader_conditional_orders(pubkeys[3])
            .capacity(32)
            .build()
            .unwrap();

        let ix = create_create_conditional_orders_account_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction(
                "create_create_conditional_orders_account_ix",
                request.clone()
            )
            .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_create_conditional_orders_account_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert(
            "CreateConditionalOrdersAccount".to_string(),
            hex_encode(&ix.data),
        );
    }
    {
        let params = PlacePositionConditionalOrderParams::builder()
            .payer(pubkeys[0])
            .trader_account(pubkeys[1])
            .position_authority(pubkeys[2])
            .perp_asset_map(pubkeys[3])
            .orderbook(pubkeys[4])
            .spline_collection(pubkeys[5])
            .global_trader_index(vec2(6, 7))
            .active_trader_buffer(vec2(8, 9))
            .trader_conditional_orders(pubkeys[10])
            .asset_id(0)
            .greater_trigger_order(TriggerOrderParams::new(
                Direction::GreaterThan,
                Side::Ask,
                StopLossOrderKind::IOC,
                1000,
                999,
            ))
            .size_base_lots(100)
            .build()
            .unwrap();

        let ix = create_place_position_conditional_order_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction(
                "create_place_position_conditional_order_ix",
                request.clone()
            )
            .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_place_position_conditional_order_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert(
            "PlacePositionConditionalOrder".to_string(),
            hex_encode(&ix.data),
        );
    }
    {
        let params = PlaceAttachedConditionalOrderParams::builder()
            .trader_account(pubkeys[1])
            .position_authority(pubkeys[2])
            .orderbook(pubkeys[4])
            .trader_conditional_orders(pubkeys[10])
            .payer(pubkeys[0])
            .global_trader_index(vec2(6, 7))
            .active_trader_buffer(vec2(8, 9))
            .order_id(FifoOrderId {
                price_in_ticks: 1000,
                order_sequence_number: 0,
            })
            .asset_id(0)
            .less_trigger_order(TriggerOrderParams::new(
                Direction::LessThan,
                Side::Ask,
                StopLossOrderKind::IOC,
                900,
                899,
            ))
            .build()
            .unwrap();

        let ix = create_place_attached_conditional_order_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction(
                "create_place_attached_conditional_order_ix",
                request.clone()
            )
            .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_place_attached_conditional_order_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert(
            "PlaceAttachedConditionalOrder".to_string(),
            hex_encode(&ix.data),
        );
    }
    {
        let order_packet = OrderPacket::limit(
            Side::Bid,
            1000,
            100,
            SelfTradeBehavior::Abort,
            None,
            client_order_id_to_bytes(0),
            None,
            OrderFlags::None,
            false,
        );
        let params = PlaceLimitOrderWithConditionalsParams::builder()
            .position_authority(pubkeys[2])
            .trader_account(pubkeys[1])
            .perp_asset_map(pubkeys[3])
            .orderbook(pubkeys[4])
            .spline_collection(pubkeys[5])
            .global_trader_index(vec2(6, 7))
            .active_trader_buffer(vec2(8, 9))
            .payer(pubkeys[0])
            .trader_conditional_orders(pubkeys[10])
            .order_packet(order_packet)
            .slot(0)
            .greater_trigger_order(TriggerOrderParams::new(
                Direction::GreaterThan,
                Side::Ask,
                StopLossOrderKind::IOC,
                1100,
                1099,
            ))
            .less_trigger_order(TriggerOrderParams::new(
                Direction::LessThan,
                Side::Ask,
                StopLossOrderKind::IOC,
                900,
                899,
            ))
            .build()
            .unwrap();

        let ix = create_place_limit_order_with_conditionals_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction(
                "create_place_limit_order_with_conditionals_ix",
                request.clone()
            )
            .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_place_limit_order_with_conditionals_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert(
            "PlaceLimitOrderWithConditionals".to_string(),
            hex_encode(&ix.data),
        );
    }
    {
        let params = CancelConditionalOrderParams::builder()
            .trader_account(pubkeys[1])
            .position_authority(pubkeys[2])
            .orderbook(pubkeys[4])
            .trader_conditional_orders(pubkeys[10])
            .conditional_order_index(1)
            .disable_first(true)
            .disable_second(false)
            .build()
            .unwrap();

        let ix = create_cancel_conditional_order_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction(
                "create_cancel_conditional_order_ix",
                request.clone()
            )
            .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_cancel_conditional_order_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("CancelConditionalOrder".to_string(), hex_encode(&ix.data));
    }
    {
        let params = DepositFundsParams::builder()
            .trader(pubkeys[0])
            .trader_account(pubkeys[1])
            .canonical_mint(pubkeys[2])
            .global_vault(pubkeys[3])
            .trader_token_account(pubkeys[4])
            .global_trader_index(vec2(5, 6))
            .active_trader_buffer(vec2(7, 8))
            .amount(1)
            .build()
            .unwrap();

        let ix = create_deposit_funds_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_deposit_funds_ix", request.clone()).unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_deposit_funds_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("DepositFunds".to_string(), hex_encode(&ix.data));
    }
    {
        let params = WithdrawFundsParams::builder()
            .trader(pubkeys[0])
            .trader_account(pubkeys[1])
            .perp_asset_map(pubkeys[2])
            .global_vault(pubkeys[3])
            .trader_token_account(pubkeys[4])
            .withdraw_queue(pubkeys[5])
            .global_trader_index(vec2(6, 7))
            .active_trader_buffer(vec2(8, 9))
            .amount(1)
            .build()
            .unwrap();

        let ix = create_withdraw_funds_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_withdraw_funds_ix", request.clone()).unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_withdraw_funds_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("WithdrawFunds".to_string(), hex_encode(&ix.data));
    }
    {
        let params = RegisterTraderParams::builder()
            .payer(pubkeys[0])
            .trader(pubkeys[1])
            .trader_account(pubkeys[2])
            .max_positions(1)
            .trader_pda_index(0)
            .subaccount_index(0)
            .build()
            .unwrap();

        let ix = create_register_trader_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_register_trader_ix", request.clone())
                .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_register_trader_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("RegisterTrader".to_string(), hex_encode(&ix.data));
    }
    {
        let params = EmberDepositParams::builder()
            .trader(pubkeys[0])
            .usdc_mint(pubkeys[1])
            .canonical_mint(pubkeys[2])
            .trader_usdc_account(pubkeys[3])
            .trader_phoenix_account(pubkeys[4])
            .amount(1)
            .build()
            .unwrap();

        let ix = create_ember_deposit_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_ember_deposit_ix", request.clone()).unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_ember_deposit_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("EmberDeposit".to_string(), hex_encode(&ix.data));
    }
    {
        let params = EmberWithdrawParams::builder()
            .trader(pubkeys[0])
            .usdc_mint(pubkeys[1])
            .canonical_mint(pubkeys[2])
            .trader_usdc_account(pubkeys[3])
            .trader_phoenix_account(pubkeys[4])
            .amount(Some(1))
            .build()
            .unwrap();

        let ix = create_ember_withdraw_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_ember_withdraw_ix", request.clone()).unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_ember_withdraw_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("EmberWithdraw".to_string(), hex_encode(&ix.data));
    }
    {
        let params = TransferCollateralParams::builder()
            .trader(pubkeys[0])
            .src_trader_account(pubkeys[1])
            .dst_trader_account(pubkeys[2])
            .perp_asset_map(pubkeys[3])
            .global_trader_index(vec2(4, 5))
            .active_trader_buffer(vec2(6, 7))
            .amount(1)
            .build()
            .unwrap();

        let ix = create_transfer_collateral_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_transfer_collateral_ix", request.clone())
                .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_transfer_collateral_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("TransferCollateral".to_string(), hex_encode(&ix.data));
    }
    {
        let params = TransferCollateralChildToParentParams::builder()
            .trader(pubkeys[0])
            .child_trader_account(pubkeys[1])
            .parent_trader_account(pubkeys[2])
            .perp_asset_map(pubkeys[3])
            .global_trader_index(vec2(4, 5))
            .active_trader_buffer(vec2(6, 7))
            .build()
            .unwrap();

        let ix = create_transfer_collateral_child_to_parent_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction(
                "create_transfer_collateral_child_to_parent_ix",
                request.clone()
            )
            .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_transfer_collateral_child_to_parent_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert(
            "TransferCollateralChildToParent".to_string(),
            hex_encode(&ix.data),
        );
    }
    {
        let params = WithdrawToTargetLeverageParams::builder()
            .trader(pubkeys[0])
            .src_trader_account(pubkeys[1])
            .dst_trader_account(pubkeys[2])
            .perp_asset_map(pubkeys[3])
            .global_trader_index(vec2(4, 5))
            .active_trader_buffer(vec2(6, 7))
            .target_leverage_bps(125_000)
            .build()
            .unwrap();

        let ix = create_withdraw_to_target_leverage_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction(
                "create_withdraw_to_target_leverage_ix",
                request.clone()
            )
            .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_withdraw_to_target_leverage_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("WithdrawToTargetLeverage".to_string(), hex_encode(&ix.data));
    }
    {
        let params = SyncParentToChildParams::builder()
            .trader_wallet(pubkeys[0])
            .parent_trader_account(pubkeys[1])
            .child_trader_account(pubkeys[2])
            .global_trader_index(vec2(3, 4))
            .build()
            .unwrap();

        let ix = create_sync_parent_to_child_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_sync_parent_to_child_ix", request.clone())
                .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_sync_parent_to_child_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("SyncParentToChild".to_string(), hex_encode(&ix.data));
    }
    {
        let params = OnboardTraderDelegatedParams::builder()
            .authority(pubkeys[0])
            .permission_account(pubkeys[1])
            .trader_account(pubkeys[2])
            .global_trader_index(vec2(3, 4))
            .active_trader_buffer(vec2(5, 6))
            .build()
            .unwrap();

        let ix = create_onboard_trader_delegated_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction(
                "create_onboard_trader_delegated_ix",
                request.clone()
            )
            .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_onboard_trader_delegated_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("OnboardTraderDelegated".to_string(), hex_encode(&ix.data));
    }
    {
        let trader_account = derive_trader_account_pda(pubkeys[0], 0, 0);
        let params = RegisterBuilderParams::builder()
            .trader_authority(pubkeys[0])
            .trader_account(trader_account)
            .trader_pda_index(0)
            .subaccount_index(0)
            .fee_bps(1)
            .build()
            .unwrap();

        let ix = create_register_builder_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_register_builder_ix", request.clone())
                .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_register_builder_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("RegisterBuilder".to_string(), hex_encode(&ix.data));
    }
    {
        let params = UpdateFeeParams::builder()
            .trader_authority(pubkeys[0])
            .fee_bps(1)
            .build()
            .unwrap();

        let ix = create_update_fee_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_update_fee_ix", request.clone()).unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_update_fee_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("UpdateFee".to_string(), hex_encode(&ix.data));
    }
    {
        let params = SetOnboarderSignerParams::builder()
            .builder_authority(pubkeys[0])
            .signer(pubkeys[1])
            .build()
            .unwrap();

        let ix = create_set_onboarder_signer_ix(params).unwrap();
        results.insert("SetOnboarderSigner".to_string(), hex_encode(&ix.data));
    }
    {
        let params = OnboardTraderParams::builder()
            .builder_authority(pubkeys[0])
            .onboarder_signer(pubkeys[1])
            .payer(pubkeys[2])
            .trader_wallet(pubkeys[3])
            .risk_authority(pubkeys[4])
            .market_authority(pubkeys[5])
            .max_positions(128)
            .trader_preference_bits(0)
            .global_trader_index(vec2(6, 7))
            .active_trader_buffer(vec2(8, 9))
            .build()
            .unwrap();

        let ix = create_onboard_trader_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_onboard_trader_ix", request.clone()).unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_onboard_trader_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("OnboardTrader".to_string(), hex_encode(&ix.data));
    }
    {
        let inner_params = LimitOrderParams::builder()
            .trader(pubkeys[0])
            .trader_account(pubkeys[1])
            .perp_asset_map(pubkeys[2])
            .orderbook(pubkeys[3])
            .spline_collection(pubkeys[4])
            .global_trader_index(vec2(5, 6))
            .active_trader_buffer(vec2(7, 8))
            .side(Side::Bid)
            .price_in_ticks(1000)
            .num_base_lots(100)
            .build()
            .unwrap();
        let inner = create_place_limit_order_ix(inner_params).unwrap();

        let params = ProxyInstructionParams::builder()
            .builder_authority(pubkeys[9])
            .builder_trader_account(pubkeys[10])
            .trader_wallet(pubkeys[0])
            .inner_instruction(inner)
            .build()
            .unwrap();

        let ix = create_proxy_instruction_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_proxy_instruction_ix", request.clone())
                .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_proxy_instruction_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("ProxyInstruction".to_string(), hex_encode(&ix.data));
    }
    {
        let inner_params = LimitOrderParams::builder()
            .trader(pubkeys[0])
            .trader_account(pubkeys[1])
            .perp_asset_map(pubkeys[2])
            .orderbook(pubkeys[3])
            .spline_collection(pubkeys[4])
            .global_trader_index(vec2(5, 6))
            .active_trader_buffer(vec2(7, 8))
            .side(Side::Bid)
            .price_in_ticks(1000)
            .num_base_lots(100)
            .build()
            .unwrap();
        let inner = create_place_limit_order_ix(inner_params).unwrap();

        let params = ProxyInstructionParams::builder()
            .builder_authority(pubkeys[9])
            .builder_trader_account(pubkeys[10])
            .trader_wallet(pubkeys[0])
            .fee_bps_override(5)
            .inner_instruction(inner)
            .build()
            .unwrap();

        let ix = create_proxy_instruction_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_proxy_instruction_ix", request.clone())
                .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_proxy_instruction_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert(
            "ProxyInstructionWithFeeOverride".to_string(),
            hex_encode(&ix.data),
        );
    }
    {
        let market_order = MarketOrderParams::builder()
            .trader(pubkeys[0])
            .trader_account(pubkeys[1])
            .perp_asset_map(pubkeys[2])
            .orderbook(pubkeys[3])
            .spline_collection(pubkeys[4])
            .global_trader_index(vec2(5, 6))
            .active_trader_buffer(vec2(7, 8))
            .side(Side::Ask)
            .num_base_lots(100)
            .min_base_lots_to_fill(0)
            .min_quote_lots_to_fill(0)
            .self_trade_behavior(SelfTradeBehavior::Abort)
            .client_order_id(0)
            .order_flags(OrderFlags::None)
            .cancel_existing(false)
            .build()
            .unwrap();
        let params = MarketOrderDelegatedParams::builder()
            .market_order(market_order)
            .trader_wallet(pubkeys[9])
            .permission_account(pubkeys[10])
            .build()
            .unwrap();

        let delegated_ix = create_place_market_order_delegated_ix(params.clone()).unwrap();
        results.insert(
            "PlaceMarketOrderDelegated".to_string(),
            hex_encode(&delegated_ix.data),
        );
    }
    {
        let params = RegisterSplineParams::builder()
            .payer(pubkeys[0])
            .authority(pubkeys[1])
            .permission_account(pubkeys[2])
            .market_account(pubkeys[3])
            .spline_collection(pubkeys[4])
            .trader_account(pubkeys[5])
            .perp_asset_map(pubkeys[6])
            .global_trader_index(vec2(7, 8))
            .active_trader_buffer(vec2(9, 10))
            .build()
            .unwrap();

        let ix = create_register_spline_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_register_spline_ix", request.clone())
                .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_register_spline_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("RegisterSpline".to_string(), hex_encode(&ix.data));
    }
    {
        let accounts = SplineUpdateAccounts::builder()
            .signer(pubkeys[0])
            .trader_account(pubkeys[1])
            .spline_collection(pubkeys[2])
            .build()
            .unwrap();
        let params = UpdateSplinePriceParams::builder()
            .new_mid_price(1000)
            .refresh_regions(true)
            .build()
            .unwrap();

        let ix = create_update_spline_price_ix(accounts, params).unwrap();
        results.insert("UpdateSplinePrice".to_string(), hex_encode(&ix.data));
    }
    {
        let accounts = SplineUpdateAccounts::builder()
            .signer(pubkeys[0])
            .trader_account(pubkeys[1])
            .spline_collection(pubkeys[2])
            .build()
            .unwrap();
        let params = UpdateSplinePriceParamsWithOrdering::builder()
            .new_mid_price(1001)
            .refresh_regions(false)
            .user_sequence_number(1)
            .client_order_id([1; 16])
            .build()
            .unwrap();

        let ix = create_update_spline_price_with_ordering_ix(accounts, params).unwrap();
        results.insert(
            "UpdateSplinePriceWithOrdering".to_string(),
            hex_encode(&ix.data),
        );
    }
    {
        let accounts = SplineUpdateAccounts::builder()
            .signer(pubkeys[0])
            .trader_account(pubkeys[1])
            .spline_collection(pubkeys[2])
            .build()
            .unwrap();
        let params = UpdateSplineParametersParamsWithOrdering::builder()
            .bid_regions(vec![
                TickRegionParams::try_from_raw(1, 3, 10, u64::MAX).unwrap(),
            ])
            .ask_regions(vec![
                TickRegionParams::try_from_raw(1, 3, 10, u64::MAX).unwrap(),
            ])
            .refresh_regions(true)
            .user_sequence_number(2)
            .client_order_id([2; 16])
            .build()
            .unwrap();

        let ix = create_update_spline_parameters_with_ordering_ix(accounts, params).unwrap();
        results.insert(
            "UpdateSplineParametersWithOrdering".to_string(),
            hex_encode(&ix.data),
        );
    }
    {
        let accounts = SplineUpdateAccounts::builder()
            .signer(pubkeys[0])
            .trader_account(pubkeys[1])
            .spline_collection(pubkeys[2])
            .build()
            .unwrap();
        let params = UpdateSplinePositionLimitsConfigParams::builder()
            .symmetric_max_position_size(100)
            .leverage_decrease_in_bps(250)
            .build()
            .unwrap();

        let ix = create_update_spline_position_limits_config_ix(accounts, params).unwrap();
        results.insert(
            "UpdateSplinePositionLimitsConfig".to_string(),
            hex_encode(&ix.data),
        );
    }
    {
        let params = DeactivateSplineParams::builder()
            .authority(pubkeys[0])
            .permission_account(pubkeys[1])
            .market_account(pubkeys[2])
            .spline_collection(pubkeys[3])
            .trader_account(pubkeys[4])
            .perp_asset_map(pubkeys[5])
            .global_trader_index(vec2(6, 7))
            .active_trader_buffer(vec2(8, 9))
            .build()
            .unwrap();

        let ix = create_deactivate_spline_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_deactivate_spline_ix", request.clone())
                .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_deactivate_spline_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("DeactivateSpline".to_string(), hex_encode(&ix.data));
    }
    {
        let params = DelegateTraderParams::builder()
            .trader_wallet(pubkeys[0])
            .trader_account(derive_trader_account_pda(pubkeys[0], 0, 0))
            .new_position_authority(pubkeys[1])
            .build()
            .unwrap();

        let ix = create_delegate_trader_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_delegate_trader_ix", request.clone())
                .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_delegate_trader_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert("DelegateTrader".to_string(), hex_encode(&ix.data));
    }
    {
        let twap_global_state = pubkeys[13];
        let twap_log_authority = pubkeys[14];
        let twap_account = pubkeys[15];
        let transfer_accounts = vec![
            AccountMeta::readonly(pubkeys[16]),
            AccountMeta::writable(pubkeys[17]),
        ];
        let order_accounts = vec![
            AccountMeta::readonly(pubkeys[18]),
            AccountMeta::writable(pubkeys[19]),
            AccountMeta::readonly_signer(pubkeys[0]),
        ];
        let child_order_packet = TwapIocOrderPacket::builder()
            .side(Side::Bid)
            .price_in_ticks(1000)
            .num_base_lots(10)
            .num_quote_lots(12_500)
            .min_base_lots_to_fill(1)
            .min_quote_lots_to_fill(1_000)
            .self_trade_behavior(SelfTradeBehavior::CancelProvide)
            .match_limit(3)
            .client_order_id(123_456_789)
            .last_valid_slot(99_999)
            .order_flags(OrderFlags::ReduceOnly)
            .cancel_existing(true)
            .build()
            .unwrap();

        let create_params = CreateTwapAccountParams::builder()
            .twap_global_state(twap_global_state)
            .twap_log_authority(twap_log_authority)
            .twap_account(twap_account)
            .trader_account(pubkeys[1])
            .payer(pubkeys[0])
            .market_id(7)
            .build()
            .unwrap();
        let create_ix = create_twap_account_ix(create_params).unwrap();
        results.insert("CreateTwapAccount".to_string(), hex_encode(&create_ix.data));

        let place_params = PlaceTwapOrderParams::builder()
            .twap_global_state(twap_global_state)
            .twap_log_authority(twap_log_authority)
            .twap_account(twap_account)
            .authority(pubkeys[0])
            .cooldown_slots(75)
            .n_child_orders(10)
            .child_order_max_slippage_bps(25)
            .child_order_min_price_in_ticks(900)
            .child_order_max_price_in_ticks(1100)
            .child_order_packet(child_order_packet)
            .child_order_collateral_quote_lots_to_transfer(5_000)
            .last_valid_slot(123_456)
            .transfer_accounts(transfer_accounts.clone())
            .order_accounts(order_accounts.clone())
            .build()
            .unwrap();
        let place_ix = create_place_twap_order_ix(place_params).unwrap();
        results.insert("PlaceTwapOrder".to_string(), hex_encode(&place_ix.data));

        let execute_params = ExecuteTwapOrderParams::builder()
            .twap_global_state(twap_global_state)
            .twap_log_authority(twap_log_authority)
            .twap_account(twap_account)
            .transfer_accounts(transfer_accounts)
            .order_accounts(order_accounts)
            .build()
            .unwrap();
        let execute_ix = create_execute_twap_order_ix(execute_params).unwrap();
        results.insert("ExecuteTwapOrder".to_string(), hex_encode(&execute_ix.data));

        let cancel_params = CancelTwapOrderParams::builder()
            .twap_global_state(twap_global_state)
            .twap_log_authority(twap_log_authority)
            .twap_account(twap_account)
            .authority(pubkeys[0])
            .build()
            .unwrap();
        let cancel_ix = create_cancel_twap_order_ix(cancel_params).unwrap();
        results.insert("CancelTwapOrder".to_string(), hex_encode(&cancel_ix.data));

        let close_params = CloseInactiveTwapAccountParams::builder()
            .twap_global_state(twap_global_state)
            .twap_log_authority(twap_log_authority)
            .twap_account(twap_account)
            .recipient(pubkeys[0])
            .build()
            .unwrap();
        let close_ix = create_close_inactive_twap_account_ix(close_params).unwrap();
        results.insert(
            "CloseInactiveTwapAccount".to_string(),
            hex_encode(&close_ix.data),
        );
    }
    {
        let market_order = MarketOrderParams::builder()
            .trader(pubkeys[0])
            .trader_account(pubkeys[1])
            .perp_asset_map(pubkeys[2])
            .orderbook(pubkeys[3])
            .spline_collection(pubkeys[4])
            .global_trader_index(vec2(5, 6))
            .active_trader_buffer(vec2(7, 8))
            .side(Side::Ask)
            .num_base_lots(100)
            .min_base_lots_to_fill(0)
            .min_quote_lots_to_fill(0)
            .self_trade_behavior(SelfTradeBehavior::Abort)
            .client_order_id(0)
            .order_flags(OrderFlags::None)
            .cancel_existing(false)
            .build()
            .unwrap();
        let delegated_params = MarketOrderDelegatedParams::builder()
            .market_order(market_order)
            .trader_wallet(pubkeys[9])
            .permission_account(pubkeys[10])
            .build()
            .unwrap();
        let inner = create_place_market_order_delegated_ix(delegated_params).unwrap();

        let params = ProxyInstructionParams::builder()
            .builder_authority(pubkeys[11])
            .builder_trader_account(pubkeys[12])
            .trader_wallet(pubkeys[9])
            .root_authority(pubkeys[0])
            .inner_instruction(inner)
            .build()
            .unwrap();

        let ix = create_proxy_instruction_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_proxy_instruction_ix", request.clone())
                .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_proxy_instruction_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert(
            "FlightProxyPlaceMarketOrderDelegated".to_string(),
            hex_encode(&ix.data),
        );
    }
    {
        let market_order = MarketOrderParams::builder()
            .trader(pubkeys[9])
            .trader_account(derive_trader_account_pda(pubkeys[0], 0, 0))
            .perp_asset_map(pubkeys[2])
            .orderbook(pubkeys[3])
            .spline_collection(pubkeys[4])
            .global_trader_index(vec2(5, 6))
            .active_trader_buffer(vec2(7, 8))
            .side(Side::Ask)
            .num_base_lots(100)
            .min_base_lots_to_fill(0)
            .min_quote_lots_to_fill(0)
            .self_trade_behavior(SelfTradeBehavior::Abort)
            .client_order_id(0)
            .order_flags(OrderFlags::None)
            .cancel_existing(false)
            .build()
            .unwrap();
        let inner = create_place_market_order_ix(market_order).unwrap();

        let params = ProxyInstructionParams::builder()
            .builder_authority(pubkeys[11])
            .builder_trader_account(pubkeys[12])
            .trader_wallet(pubkeys[9])
            .root_authority(pubkeys[0])
            .inner_instruction(inner)
            .build()
            .unwrap();

        let ix = create_proxy_instruction_ix(params.clone()).unwrap();
        let request = serde_json::to_value(&params).unwrap();
        assert_eq!(
            aux_rs_phoenix::build_instruction("create_proxy_instruction_ix", request.clone())
                .unwrap(),
            serde_json::to_value(&ix).unwrap()
        );
        dispatch.insert(
            "create_proxy_instruction_ix",
            serde_json::json!({"request":request,"instruction":ix}),
        );
        results.insert(
            "FlightProxyPlaceMarketOrderPositionAuthority".to_string(),
            hex_encode(&ix.data),
        );
    }
    if std::env::var_os("AUX_UPDATE_INSTRUCTION_FIXTURES").is_some() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/instruction_dispatch.json");
        let mut fixtures: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        for (name, fixture) in dispatch {
            fixtures[name] = fixture;
        }
        std::fs::write(
            path,
            serde_json::to_string_pretty(&fixtures).unwrap() + "\n",
        )
        .unwrap();
    }
    let reference: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/instructions.json")).unwrap();
    assert!(results.len() >= 25);
    for (name, bytes) in results {
        assert_eq!(reference[&name], bytes, "{name}");
    }
}
fn hex_encode(bytes: &[u8]) -> String {
    aux_rs_core::hex::encode(bytes)
}

fn derive_trader_account_pda(authority: Pubkey, pda_index: u8, subaccount_index: u8) -> Pubkey {
    aux_rs_phoenix::trader_address(&authority.to_string(), pda_index, subaccount_index)
        .unwrap()
        .parse()
        .unwrap()
}
