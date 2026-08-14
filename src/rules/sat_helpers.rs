#[derive(Debug)]
pub(crate) struct SatVariableAllocator {
    reduction: &'static str,
    next: u64,
}

impl SatVariableAllocator {
    pub(crate) fn new(reduction: &'static str, existing: usize) -> Result<Self, String> {
        if existing > i32::MAX as usize {
            return Err(format!(
                "{reduction} has {existing} source variables; SAT variable numbers are limited to {}",
                i32::MAX
            ));
        }
        Ok(Self {
            reduction,
            next: u64::try_from(existing).expect("usize SAT count fits u64") + 1,
        })
    }

    pub(crate) fn allocate(&mut self) -> Result<i32, String> {
        let variable = self.next;
        if variable > i32::MAX as u64 {
            return Err(format!(
                "{} cannot allocate 1 auxiliary variable after {}; SAT variable numbers are limited to {}",
                self.reduction,
                self.num_vars(),
                i32::MAX
            ));
        }
        self.next += 1;
        Ok(i32::try_from(variable).expect("checked SAT variable fits i32"))
    }

    pub(crate) fn allocate_many(&mut self, count: usize) -> Result<Vec<i32>, String> {
        if count == 0 {
            return Ok(Vec::new());
        }
        let count = u64::try_from(count).expect("usize allocation count fits u64");
        let last = self
            .next
            .checked_add(count - 1)
            .ok_or_else(|| format!("{} auxiliary variable count overflow", self.reduction))?;
        if last > i32::MAX as u64 {
            return Err(format!(
                "{} cannot allocate {count} auxiliary variables after {}; SAT variable numbers are limited to {}",
                self.reduction,
                self.num_vars(),
                i32::MAX
            ));
        }
        let variables = (self.next..=last)
            .map(|variable| i32::try_from(variable).expect("checked SAT variable fits i32"))
            .collect();
        self.next = last + 1;
        Ok(variables)
    }

    pub(crate) fn num_vars(&self) -> usize {
        usize::try_from(self.next - 1).expect("SAT variable count fits usize")
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/sat_helpers.rs"]
mod tests;
