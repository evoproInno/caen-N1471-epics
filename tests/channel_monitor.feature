# Copyright (c) 2024 evopro Innovation

Feature: Channel level monitoring
    The IOC supports reading out channel level information

    "module" refers to the Caen N1471A power supply
    "client" refers to channel access or pv access client

    Scenario: Read out VSET
        Given The module is accessible

        When The client read the VSET value for <channel>

        Then The answer should contain one floating point value

        Examples:
        | channel |
        |    0    |
        |    1    |

    Scenario: Read out VMON
        Given The module is accessible

        When The client read the VMON value for <channel>

        Then The answer should contain one floating point value

        Examples:
        | channel |
        |    0    |
        |    1    |

    Scenario: Read out ISET
        Given The module is accessible

        When The client read the ISET value for <channel>

        Then The answer should contain one floating point value

        Examples:
        | channel |
        |    0    |
        |    1    |

    Scenario: Read out IMON
        Given The module is accessible

        When The client read the IMON value for <channel>

        Then The answer should contain one floating point value

        Examples:
        | channel |
        |    0    |
        |    1    |

    Scenario: Read out IMON RANGE
        Given The module is accessible

        When The client read the IMON RANGE value for <channel>

        Then The answer should be 'HIGH'

        Examples:
        | channel |
        |    0    |
        |    1    |

    Scenario: Read out MAXVSET
        Given The module is accessible

        When The client read the MAX VSET value for <channel>

        Then The answer should contain one integer value

        Examples:
        | channel |
        |    0    |
        |    1    |

    Scenario: Read out RAMP UP
        Given The module is accessible

        When The client read the RAMP UP value for <channel>

        Then The answer should contain one integer value

        Examples:
        | channel |
        |    0    |
        |    1    |

    Scenario: Read out RAMP DOWN
        Given The module is accessible

        When The client read the RAMP DOWN value for <channel>

        Then The answer should contain one integer value

        Examples:
        | channel |
        |    0    |
        |    1    |

    Scenario: Read out TRIP
        Given The module is accessible

        When The client read the TRIP value for <channel>

        Then The answer should contain one floating point value

        Examples:
        | channel |
        |    0    |
        |    1    |

    Scenario: Read out POWER DOWN
        Given The module is accessible

        When The client read the POWER DOWN value for <channel>

        Then The answer should be 'KILL' or 'RAMP'

        Examples:
        | channel |
        |    0    |
        |    1    |

    Scenario: Read out POL
        Given The module is accessible

        When The client read the POL value for <channel>

        Then The answer should be '+' or '-'

        Examples:
        | channel |
        |    0    |
        |    1    |

    Scenario: Read out status bits
        Given The module is accessible

        When The client read the <name> value for <channel>

        Then The answer should be 'YES' or 'NO'

        Examples:
        |         name          | channel |
        | OnOff                 |    0    |
        | RampUpStatus          |    0    |
        | RampDownStatus        |    0    |
        | OverCurrent           |    0    |
        | OverVoltage           |    0    |
        | UnderVoltage          |    0    |
        | MaxVoltageProtection  |    0    |
        | Tripped               |    0    |
        | OverPower             |    0    |
        | OverTemperature       |    0    |
        | Disabled              |    0    |
        | Killed                |    0    |
        | Interlock             |    0    |
        | CalibrationError      |    0    |
        | OnOff                 |    1    |
        | RampUpStatus          |    1    |
        | RampDownStatus        |    1    |
        | OverCurrent           |    1    |
        | OverVoltage           |    1    |
        | UnderVoltage          |    1    |
        | MaxVoltageProtection  |    1    |
        | Tripped               |    1    |
        | OverPower             |    1    |
        | OverTemperature       |    1    |
        | Disabled              |    1    |
        | Killed                |    1    |
        | Interlock             |    1    |
        | CalibrationError      |    1    |

