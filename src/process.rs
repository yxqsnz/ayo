use crate::{
    config::{Config, Group},
    util,
};
use ioprio::{Class, Pid, Priority, Target};
use procfs::process::{all_processes, Process};
pub(crate) fn process(config: &Config) -> anyhow::Result<()> {
    for proc in all_processes()?.filter_map(|p| p.ok()) {
        if let Some(cmdline) = proc.cmdline()?.first() {
            for rule in &config.rules {
                if rule.process.is_match(cmdline) {
                    let group = config
                        .groups
                        .get(&rule.group)
                        .ok_or_else(|| anyhow::format_err!("Unknown group: {}", rule.group))?;

                    if let Err(err) = apply(&proc, group.to_owned()) {
                        log::warn!(
                            "Failed to apply rules for {} ({}). Reason: {}",
                            cmdline,
                            proc.pid(),
                            err
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

fn apply(proc: &Process, group: Group) -> anyhow::Result<()> {
    let stat = proc.stat()?;
    let target = Target::Process(Pid::from_raw(proc.pid));
    let proc_prio = ioprio::get_priority(target)?;
    let class: Class = group
        .io_class
        .try_into()
        .map_err(|_| anyhow::format_err!("Invalid Priority"))?;

    if stat.nice != group.nice {
        util::set_nice(proc.pid() as u32, group.nice);
    }

    if proc_prio.class() != Some(class) {
        ioprio::set_priority(target, Priority::new(class))?;
    }

    Ok(())
}
