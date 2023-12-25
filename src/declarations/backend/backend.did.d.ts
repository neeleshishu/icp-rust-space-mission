import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type Error = { 'NotFound' : { 'msg' : string } };
export type Result = { 'Ok' : bigint } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : SpaceMission } |
  { 'Err' : Error };
export interface SpaceMission {
  'id' : bigint,
  'destination' : string,
  'description' : string,
  'mission_name' : string,
  'launch_date' : bigint,
  'estimated_arrival_date' : bigint,
}
export interface SpaceMissionPayload {
  'destination' : string,
  'description' : string,
  'mission_name' : string,
  'launch_date' : bigint,
  'estimated_arrival_date' : bigint,
}
export interface _SERVICE {
  'add_space_mission' : ActorMethod<[SpaceMissionPayload], [] | [SpaceMission]>,
  'calculate_remaining_days_for_arrival' : ActorMethod<[bigint], Result>,
  'delete_space_mission' : ActorMethod<[bigint], Result_1>,
  'find_closest_space_mission' : ActorMethod<[bigint], [] | [SpaceMission]>,
  'get_all_space_missions' : ActorMethod<[], Array<SpaceMission>>,
  'get_space_mission' : ActorMethod<[bigint], Result_1>,
  'get_space_missions_before_date' : ActorMethod<[bigint], Array<SpaceMission>>,
  'get_space_missions_by_destination' : ActorMethod<
    [string],
    Array<SpaceMission>
  >,
  'get_space_missions_count_before_date' : ActorMethod<[bigint], bigint>,
  'get_total_space_missions' : ActorMethod<[], bigint>,
  'update_estimated_arrival_date' : ActorMethod<[bigint, bigint], Result_1>,
  'update_launch_date' : ActorMethod<[bigint, bigint], Result_1>,
  'update_space_mission' : ActorMethod<[bigint, SpaceMissionPayload], Result_1>,
}
