#include <stdio.h>
#include <string.h>

typedef enum{
    STATE_Cruise,
    STATE_AtmosphericEntry,
    STATE_ParachuteDeploy,
    STATE_BackupChute,
    STATE_PoweredDescent,
    STATE_Landing,
    STATE_Safe,
    STATE_Crash,
    STATE_BurnUp,
    STATE_Finished,
} State;

void print_available_events(State s) {
     printf("   [Options: ");
     switch (s) {
        case STATE_Cruise:
            printf("EntryInterface");
            break;
        case STATE_AtmosphericEntry:
            printf("HeatShieldFailure, StableDescend, Turbulence");
            break;
        case STATE_ParachuteDeploy:
            printf("ChuteFailure, LowAltitude");
            break;
        case STATE_BackupChute:
            printf("ChuteFailure, LowAltitude");
            break;
        case STATE_PoweredDescent:
            printf("EngineFailure, FuelDepleted, Correction");
            break;
        case STATE_Landing:
            printf("Touchdown, TipOver");
            break;
        case STATE_Safe:
            printf("Shutdown");
            break;
        case STATE_Crash:
            printf("RecoveryAttempt");
            break;
        case STATE_BurnUp:
            printf("");
            break;
        case STATE_Finished:
            printf("");
            break;
    }
    printf("]\n");
}

int is_terminal(State s) {
    switch (s) {
        case STATE_BurnUp: return 1;
        case STATE_Finished: return 1;
        default: return 0;
    }
    return 0;
}

State next_state(State current, const char* event) {
    switch (current) {
        case STATE_Cruise:
            if (strcmp(event, "EntryInterface") == 0) return STATE_AtmosphericEntry;
            break;
        case STATE_AtmosphericEntry:
            if (strcmp(event, "HeatShieldFailure") == 0) return STATE_BurnUp;
            if (strcmp(event, "StableDescend") == 0) return STATE_ParachuteDeploy;
            if (strcmp(event, "Turbulence") == 0) return STATE_AtmosphericEntry;
            break;
        case STATE_ParachuteDeploy:
            if (strcmp(event, "ChuteFailure") == 0) return STATE_BackupChute;
            if (strcmp(event, "LowAltitude") == 0) return STATE_PoweredDescent;
            break;
        case STATE_BackupChute:
            if (strcmp(event, "ChuteFailure") == 0) return STATE_Crash;
            if (strcmp(event, "LowAltitude") == 0) return STATE_PoweredDescent;
            break;
        case STATE_PoweredDescent:
            if (strcmp(event, "EngineFailure") == 0) return STATE_Crash;
            if (strcmp(event, "FuelDepleted") == 0) return STATE_Landing;
            if (strcmp(event, "Correction") == 0) return STATE_PoweredDescent;
            break;
        case STATE_Landing:
            if (strcmp(event, "Touchdown") == 0) return STATE_Safe;
            if (strcmp(event, "TipOver") == 0) return STATE_Crash;
            break;
        case STATE_Safe:
            if (strcmp(event, "Shutdown") == 0) return STATE_Finished;
            break;
        case STATE_Crash:
            if (strcmp(event, "RecoveryAttempt") == 0) return STATE_Cruise;
            break;
        case STATE_BurnUp:
            break;
        case STATE_Finished:
            break;
    }
    return (State)-1;
}

const char* state_name(State s) {
     switch (s) {
        case STATE_Cruise: return "Cruise";
        case STATE_AtmosphericEntry: return "AtmosphericEntry";
        case STATE_ParachuteDeploy: return "ParachuteDeploy";
        case STATE_BackupChute: return "BackupChute";
        case STATE_PoweredDescent: return "PoweredDescent";
        case STATE_Landing: return "Landing";
        case STATE_Safe: return "Safe";
        case STATE_Crash: return "Crash";
        case STATE_BurnUp: return "BurnUp";
        case STATE_Finished: return "Finished";
    }
    return "Unknown";
}

int main() {
    State current = 0; // Assuming 0 is initial
    char buffer[100];
    
    printf("FSM Started...\n");

    while(1) {
        printf("   Current State: %s\n", state_name(current));

        if (is_terminal(current)) {
            printf(">> Final state reached. Terminating.\n");
            break;
        }
        
        print_available_events(current);

        printf(">> ");
        if (scanf("%99s", buffer) != 1) break;
        
        State next = next_state(current, buffer);
        if (next != (State)-1) {
            printf(">> Transitioned: %s -> %s\n", state_name(current), state_name(next));
            current = next;
        } else {
            printf(">> Invalid event. Stayed in %s.\n", state_name(current));
        }
    }
    return 0;
}
