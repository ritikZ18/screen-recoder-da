# Maintenance Checklist

## Daily Checks

- [ ] Verify application starts without errors
- [ ] Check for critical log errors
- [ ] Monitor disk space for output directory

## Weekly Checks

- [ ] Review application logs for warnings
- [ ] Clean old build artifacts (`cargo clean`)
- [ ] Check for dependency updates (`npm outdated`, `cargo outdated`)
- [ ] Verify recordings are being saved correctly
- [ ] Test recording functionality

## Monthly Checks

- [ ] Update dependencies (Rust and Node.js)
- [ ] Run security audit (`npm audit`)
- [ ] Clean old recordings (>30 days)
- [ ] Review and rotate logs
- [ ] Check disk space usage
- [ ] Review performance metrics
- [ ] Update documentation if needed

## Quarterly Checks

- [ ] Major dependency updates
- [ ] Review and update code quality
- [ ] Performance benchmarking
- [ ] Security review
- [ ] Backup important configurations
- [ ] Review and update technical documentation

## Before Release

- [ ] Run full test suite
- [ ] Update version numbers
- [ ] Update CHANGELOG
- [ ] Review and update documentation
- [ ] Code review
- [ ] Performance testing
- [ ] Security audit
- [ ] Build and test on all target platforms
- [ ] Create release notes

## Emergency Procedures

### Application Crashes

1. Check logs for error messages
2. Verify system resources (CPU, memory, disk)
3. Check for conflicting applications
4. Restart application
5. If persistent, check GitHub issues or documentation

### Performance Degradation

1. Check system resource usage
2. Review recent changes
3. Check for memory leaks (monitor memory over time)
4. Review encoding settings
5. Consider reducing quality/resolution

### Data Loss

1. Check output directory permissions
2. Verify disk space
3. Check for disk errors
4. Review logs for I/O errors
5. Restore from backup if available

### Security Issues

1. Review security advisories
2. Update dependencies immediately
3. Review access logs
4. Check for unauthorized access
5. Update security documentation

---

**Note**: Adjust frequency based on usage patterns and requirements.

