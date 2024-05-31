# Copyright (c) 2024 evopro Innovation

Feature: Module level monitoring
    The IOC supports reading out module level information

    "module" refers to the Caen N1471 power supply
    "client" refers to channel access or pv access client

    Scenario: Read module name
        Given The module is accessible

        When The client read the module name

        Then The answer should be equal to 'N1471A'

    Scenario: Read number of channels
        Given The module is accessible

        When The client read the number of channels

        Then The answer should be equal to 2

    Scenario: Read firmware release
        Given The module is accessible

        When The client read the firmware release

        Then The answer should not be empty

    Scenario: Read serial number
        Given The module is accessible

        When The client read the serial number

        Then The answer should be formatted as 'XXXXX'

    Scenario: Read interlock status
        Given The module is accessible

        When The client read the interlock status

        Then The answer should be 'YES' or 'NO'

    Scenario: Read interlock mode
        Given The module is accessible

        When The client read the interlock mode

        Then The answer should be 'OPEN' or 'CLOSED'

    Scenario: Read control mode
        Given The module is accessible

        When The client read the control mode

        Then The answer should be 'LOCAL' or 'REMOTE'

    Scenario: Read local bus termination
        Given The module is accessible

        When The client read the local bus termination

        Then The answer should be 'ON' or 'OFF'

    Scenario: Read channel in alarm status from board alarm status
        Given The module is accessible

        When The client read the channel alarm status for channel <index>

        Then The answer should be 'YES' or 'NO'

        Examples:
        | index |
        |   0   |
        |   1   |
        |   2   |
        |   3   |

    Scenario: Read board in POWER FAIL alarm status
        Given The module is accessible

        When The client read the board in POWER FAIL alarm status

        Then The answer should be 'YES' or 'NO'

    Scenario: Read board in OVER POWER alarm status
        Given The module is accessible

        When The client read the board in OVER POWER alarm status

        Then The answer should be 'YES' or 'NO'

    Scenario: Read Internal HV Clock FAIL alarm status
        Given The module is accessible

        When The client read the Internal HV Clock FAIL alarm status

        Then The answer should be 'YES' or 'NO'