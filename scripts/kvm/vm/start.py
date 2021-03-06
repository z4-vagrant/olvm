#!/usr/bin/python2

import sys
import time
import json
import subprocess

vm = json.loads(sys.argv[1])
params = vm['parameters']
ifaces = vm['interfaces']

folder = '/var/lib/olvm/vms/kvm/' + vm['name']
monitor = folder + '/monitor.sock'
disk = folder + '/disk.data'

opts = [
    'qemu-system-x86_64',
    '-nographic',
    '-drive', 'file=' + disk,
    '-qmp', 'unix:' + monitor + ',server,nowait'
]

if 'acceleration' in params:
    opts.append('-enable-kvm')

if 'cpus' in params:
    opts.append('-smp')
    opts.append(params['cpus'])

if 'memory' in params:
    opts.append('-m')
    opts.append(params['memory'])

if 'vnc' in params:
    opts.append('-k')
    opts.append('fr')
    opts.append('-vnc')

    if 'vncws' in params:
        opts.append(params['vnc'] + ',websocket=' + params['vncws'])
    else:
        opts.append(params['vnc'])

if 'cdrom' in params:
    opts.append('-cdrom')
    opts.append(params['cdrom'])

if 'args' in params:
    opts.append(params['args'])

if len(ifaces) > 0:
    index = 0
    for iface in ifaces:
        opts.append('-netdev')
        opts.append('tap,id=net' + str(index) + ',ifname=vm' + vm['name'] + '.' + str(index))

        opts.append('-device')
        opts.append('driver=virtio-net,netdev=net' + str(index) + ',mac=' + iface['mac'])

        index = index + 1

child = subprocess.Popen(opts, stdout = subprocess.PIPE, stderr = subprocess.PIPE)

time.sleep(2)
child.poll()

if child.returncode is None:
    print 'pid', child.pid
    sys.exit(0)
else:
    stdout, stderr = child.communicate()

    sys.stderr.write('qemu exited: ' + stderr)
    sys.exit(1)
