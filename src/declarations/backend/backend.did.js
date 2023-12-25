export const idlFactory = ({ IDL }) => {
  const SpaceMissionPayload = IDL.Record({
    'destination' : IDL.Text,
    'description' : IDL.Text,
    'mission_name' : IDL.Text,
    'launch_date' : IDL.Nat64,
    'estimated_arrival_date' : IDL.Nat64,
  });
  const SpaceMission = IDL.Record({
    'id' : IDL.Nat64,
    'destination' : IDL.Text,
    'description' : IDL.Text,
    'mission_name' : IDL.Text,
    'launch_date' : IDL.Nat64,
    'estimated_arrival_date' : IDL.Nat64,
  });
  const Error = IDL.Variant({ 'NotFound' : IDL.Record({ 'msg' : IDL.Text }) });
  const Result = IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : Error });
  const Result_1 = IDL.Variant({ 'Ok' : SpaceMission, 'Err' : Error });
  return IDL.Service({
    'add_space_mission' : IDL.Func(
        [SpaceMissionPayload],
        [IDL.Opt(SpaceMission)],
        [],
      ),
    'calculate_remaining_days_for_arrival' : IDL.Func(
        [IDL.Nat64],
        [Result],
        ['query'],
      ),
    'delete_space_mission' : IDL.Func([IDL.Nat64], [Result_1], []),
    'find_closest_space_mission' : IDL.Func(
        [IDL.Nat64],
        [IDL.Opt(SpaceMission)],
        ['query'],
      ),
    'get_all_space_missions' : IDL.Func([], [IDL.Vec(SpaceMission)], ['query']),
    'get_space_mission' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'get_space_missions_before_date' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(SpaceMission)],
        ['query'],
      ),
    'get_space_missions_by_destination' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(SpaceMission)],
        ['query'],
      ),
    'get_space_missions_count_before_date' : IDL.Func(
        [IDL.Nat64],
        [IDL.Nat64],
        ['query'],
      ),
    'get_total_space_missions' : IDL.Func([], [IDL.Nat64], ['query']),
    'update_estimated_arrival_date' : IDL.Func(
        [IDL.Nat64, IDL.Nat64],
        [Result_1],
        [],
      ),
    'update_launch_date' : IDL.Func([IDL.Nat64, IDL.Nat64], [Result_1], []),
    'update_space_mission' : IDL.Func(
        [IDL.Nat64, SpaceMissionPayload],
        [Result_1],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
