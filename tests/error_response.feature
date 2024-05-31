# Copyright (c) 2024 evopro Innovation

Feature: Error response
    The IOC handles error reponses from the HV PSU

    "module" refers to the Caen N1471 power supply
    "client" refers to channel access or pv access client

    Scenario: Local mode error
        Given The module is accessible
        And Interlock mode is CLOSED
        And The module is in local control mode

        When The client set interlock mode to OPEN

        Then Interlock mode should be CLOSED
        And Control mode Error should be set

    Scenario: Channel error
        Given The module is accessible
        And The module has <ch_max> channels

        When The client read VSET value on channel <ch_index>

        Then Wrong channel Error should be set

        Examples:
        | ch_max | ch_index |
        |    1   |     1    |
        |    2   |     2    |
        |    3   |     3    |
        |    4   |     4    |
