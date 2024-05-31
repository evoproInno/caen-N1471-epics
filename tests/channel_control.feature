# Copyright (c) 2024 evopro Innovation

Feature: Channel level control
    The IOC can set channel level parameters

    <set interval> defines the maximum acceptable time for a request to take effect
    available control parameters: VSET, ISET, MAXVSET, RAMP UP, RAMP DOWN, TRIP TIME
    <channel id> = 0, 1, 2, 3
    "module" refers to the Caen N1471 power supply
    "client" refers to channel access or pv access client

    Scenario: Set MAXVSET value to a valid value
        Given The module is accessible
        And The module is in remote control mode
        And The MAXVSET value is 8100V for all channels

        When The client set the MAXVSET to 1000V for channel 0

        Then MAXVSET should be 1000V for channel 0 within <set interval>
        And MAXVSET should be 8100V for <channel id>
        And MAXVSET should be 8100V for channel 2
        And MAXVSET should be 8100V for channel 3

    Scenario: Set MAXVSET value below its minimum
        Given The module is accessible
        And The module is in remote control mode
        And The MAXVSET minimum value is 0V for all channels

        When The client set the MAXVSET to -1V for channel 0

        Then MAXVSET should be 0V for channel 0
        And MAXVSET should be 0V for <channel id>
        And MAXVSET should be 0V for channel 2
        And MAXVSET should be 0V for channel 3
        And Value Error should be set

    Scenario: Set MAXVSET value above its maximum
        Given The module is accessible
        And The module is in remote control mode
        And The MAXVSET maximum value is 8100V for all channels

        When The client set the MAXVSET to 10000V for channel 0

        Then MAXVSET should be 8100V for channel 0
        And MAXVSET should be 8100V for <channel id>
        And MAXVSET should be 8100V for channel 2
        And MAXVSET should be 8100V for channel 3
        And Value Error should be set

    Scenario: Set POWER DOWN mode
        Given The module is accessible
        And The module is in remote control mode

        When The client set the POWER DOWN mode to <mode> for channel 0

        Then The POWER DOWN mode should be <mode> for channel 0 within <set interval>

        |  <mode>  |
        |   RAMP   |
        |   KILL   |

    Scenario: Set Channel ON
        Given The module is accessible
        And The module is in remote control mode
        And Channel 0 is OFF

        When The client set Channel 0 ON

        Then Channel 0 should be ON within <set interval>

    Scenario: Set Channel OFF
        Given The module is accessible
        And The module is in remote control mode
        And Channel 0 is ON

        When The client set Channel 0 OFF

        Then Channel 0 should be OFF within <set interval>